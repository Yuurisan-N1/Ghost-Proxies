use serde_json::Value;

use crate::net::client::{code_of, message_of, Client};
use crate::support::consts::net::{PATH_SEND_VERIFY, PATH_VERIFY_OTP};
use crate::support::error::{refusal, AppError, AppResult};
use crate::wire::shape;

pub async fn send(client: &Client, email: &str) -> AppResult<()> {
    let body = shape::send_verify(email);
    let (status, value) = client.post(PATH_SEND_VERIFY, &body).await?;
    if status.is_success() {
        return Ok(());
    }
    Err(AppError::api(refusal(&message_of(&value, ""))))
}

pub async fn confirm(client: &Client, email: &str, otp: &str) -> AppResult<()> {
    let code = digits(otp);
    if code.len() != 6 {
        return Err(AppError::api("the code must be six digits"));
    }
    let body = shape::verify_otp(email, &code);
    let (status, value) = client.post(PATH_VERIFY_OTP, &body).await?;
    if status.is_success() {
        let verified = value
            .get("user")
            .and_then(|user| user.get("emailVerified"))
            .and_then(Value::as_bool);
        return match verified {
            Some(false) => Err(AppError::api(
                "the code was accepted but the mailbox is not marked verified",
            )),
            _ => Ok(()),
        };
    }
    if code_of(&value).contains("INVALID") {
        return Err(AppError::api("the code was refused"));
    }
    Err(AppError::api(refusal(&message_of(&value, ""))))
}

pub fn digits(raw: &str) -> String {
    raw.chars().filter(|c| c.is_ascii_digit()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_code_keeps_only_digits() {
        assert_eq!(digits(" 12 34-56 "), "123456");
    }

    #[test]
    fn a_code_with_letters_loses_them() {
        assert_eq!(digits("12a34b56"), "123456");
    }

    #[test]
    fn an_empty_code_reads_empty() {
        assert_eq!(digits(""), "");
    }
}
