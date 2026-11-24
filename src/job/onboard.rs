use std::collections::BTreeMap;

use crate::auth::create::{self, Standing};
use crate::auth::verify;
use crate::inbox::prompt::{ask_otp, Answer};
use crate::job::store::{self, Line, Record};
use crate::job::Env;
use crate::keys::list::{self, Key};
use crate::net::client::Client;
use crate::net::egress::{self, Proxy};
use crate::security::fingerprint;
use crate::support::consts::net::{BASE_URL, COUNTRIES};
use crate::support::device::Device;
use crate::support::error::AppResult;
use crate::support::log::format::{lg, ly, mask_key};

pub enum Outcome {
    Stored,
    Skipped(String),
}

pub fn country_for(index: usize) -> &'static str {
    COUNTRIES[index % COUNTRIES.len()]
}

pub async fn run(
    env: &Env,
    state: &mut BTreeMap<String, Record>,
    line: &Line,
    index: usize,
) -> AppResult<Outcome> {
    let country = country_for(index);
    let pool = egress::load(&env.proxy)?;
    let proxy: Option<Proxy> = egress::pick(&pool, index).map(|found| found.with_country(country));

    let device = match state.get(&line.email) {
        Some(record) => record.device.clone(),
        None => Device::fresh(country),
    };

    let client = Client::open(BASE_URL, proxy.as_ref(), &device)?;

    if let Some(found) = proxy.as_ref() {
        ly(&format!("Using proxy {}", found.masked()));
    }
    if state.get(&line.email).is_some() {
        ly("The cached device identity was reused for this account");
    } else {
        ly("A new device identity was created and cached for this account");
    }

    let password = match &line.password {
        Some(found) => found.clone(),
        None => fingerprint::password(),
    };

    let standing = create::sign_in(&client, &line.email, &password).await?;
    let standing = match standing {
        Standing::Signed => Standing::Signed,
        Standing::Unverified => {
            ly("The mailbox is not verified yet so a code is sent");
            verify::send(&client, &line.email).await?;
            match settle(&client, &line.email, &password).await? {
                Settle::Verified => Standing::Signed,
                Settle::Skip(reason) => return Ok(Outcome::Skipped(reason)),
            }
        }
        Standing::Missing => {
            ly("The account is new so the trial gate is checked first");
            create::gate(&client, &line.email).await?;
            match create::sign_up(&client, &line.email, &password).await? {
                Standing::Signed => Standing::Signed,
                Standing::Unverified => {
                    ly("The sign up already sent a code so no resend is made");
                    match settle(&client, &line.email, &password).await? {
                        Settle::Verified => Standing::Signed,
                        Settle::Skip(reason) => return Ok(Outcome::Skipped(reason)),
                    }
                }
                Standing::Missing => {
                    return Ok(Outcome::Skipped(
                        "the server did not create the account".to_string(),
                    ))
                }
                Standing::Refused(reason) => return Ok(Outcome::Skipped(reason)),
            }
        }
        Standing::Refused(reason) => return Ok(Outcome::Skipped(reason)),
    };

    if standing != Standing::Signed {
        return Ok(Outcome::Skipped(
            "the mailbox stayed unverified".to_string(),
        ));
    }

    let keys = list::fetch(&client).await?;
    let key: Key = match list::default_of(&keys) {
        Some(found) => found,
        None => return Ok(Outcome::Skipped("the account holds no api key".to_string())),
    };

    store::append_key(&env.apikey, &key.value)?;
    state.insert(
        line.email.clone(),
        Record {
            password: password.clone(),
            apikey: key.value.clone(),
            created_at: key.created_at.clone(),
            device,
        },
    );
    store::save_state(&env.state, state)?;

    lg(&format!(
        "The account now holds the key {}",
        mask_key(&key.value)
    ));

    Ok(Outcome::Stored)
}

enum Settle {
    Verified,
    Skip(String),
}

async fn settle(client: &Client, email: &str, password: &str) -> AppResult<Settle> {
    loop {
        match ask_otp(email) {
            Answer::Code(code) => {
                if code.len() != 6 {
                    return Ok(Settle::Skip("no code was entered".to_string()));
                }
                verify::confirm(client, email, &code).await?;
                break;
            }
            Answer::Resend => {
                verify::send(client, email).await?;
                ly("A fresh code was sent to the mailbox");
            }
            Answer::Skip => {
                return Ok(Settle::Skip(
                    "the code was not entered for this account".to_string(),
                ))
            }
            Answer::Stop => crate::job::stop::now(),
        }
    }
    lg("The account was verified by the server");
    let standing = create::sign_in(client, email, password).await?;
    if standing == Standing::Signed {
        Ok(Settle::Verified)
    } else {
        Ok(Settle::Skip("the mailbox stayed unverified".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_country_rotates_with_the_index() {
        assert_eq!(country_for(0), COUNTRIES[0]);
        assert_eq!(country_for(1), COUNTRIES[1]);
        assert_eq!(country_for(COUNTRIES.len()), COUNTRIES[0]);
    }
}
