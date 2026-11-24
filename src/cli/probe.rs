use crate::job::Env;
use crate::net::client::Client;
use crate::net::egress;
use crate::security::signup_check;
use crate::support::consts::net::BASE_URL;
use crate::support::device::Device;
use crate::support::log::format::{gap, lg, lr, ly};

pub async fn run(env: &Env) {
    let pool = match egress::load(&env.proxy) {
        Ok(pool) => pool,
        Err(err) => {
            lr(&err.to_string());
            return;
        }
    };

    if let Some(found) = pool.first() {
        ly(&format!("Using proxy {}", found.masked()));
    }

    let lines = match crate::job::store::read_lines(&env.data) {
        Ok(lines) => lines,
        Err(err) => {
            lr(&err.to_string());
            return;
        }
    };
    let email = match lines.first() {
        Some(line) => line.email.clone(),
        None => {
            lr("File data.txt is empty and holds no email password pairs");
            return;
        }
    };

    let device = Device::fresh("us");
    let proxy = egress::pick(&pool, 0).map(|found| found.with_country("us"));
    let client = match Client::open(BASE_URL, proxy.as_ref(), &device) {
        Ok(client) => client,
        Err(err) => {
            lr(&err.to_string());
            return;
        }
    };

    match signup_check::check(&client, &email).await {
        Ok(verdict) => {
            if verdict.allowed {
                lg("The trial is still open on this line");
            } else {
                lr("The trial was already claimed on this device or network");
            }
        }
        Err(err) => lr(&err.to_string()),
    }
    gap();
}
