use serde_json::Value;

use crate::net::client::{message_of, Client};
use crate::support::consts::net::PATH_CHECK_SIGNUP;
use crate::support::error::{refusal, AppError, AppResult};
use crate::wire::shape;

pub struct Verdict {
    pub allowed: bool,
    pub reason: String,
}

pub async fn check(client: &Client, email: &str) -> AppResult<Verdict> {
    let body = shape::check_signup(email, client.device());
    let (status, value) = client.post(PATH_CHECK_SIGNUP, &body).await?;

    if !status.is_success() {
        return Err(AppError::api(refusal(&message_of(&value, ""))));
    }

    let allowed = value
        .get("allowed")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    Ok(Verdict {
        allowed,
        reason: reason_of(&value),
    })
}

fn reason_of(value: &Value) -> String {
    for key in ["reason", "message", "error"] {
        if let Some(found) = value.get(key).and_then(Value::as_str) {
            if !found.trim().is_empty() {
                return found.to_string();
            }
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_reason_prefers_the_reason_field() {
        let value: Value = serde_json::json!({"reason": "first", "message": "second"});
        assert_eq!(reason_of(&value), "first");
    }

    #[test]
    fn the_reason_falls_back_to_the_message() {
        let value: Value = serde_json::json!({"message": "second"});
        assert_eq!(reason_of(&value), "second");
    }

    #[test]
    fn a_missing_reason_reads_empty() {
        assert_eq!(reason_of(&serde_json::json!({})), "");
    }
}
