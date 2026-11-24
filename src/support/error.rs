use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Config(String),
    #[error("{0}")]
    Network(String),
    #[error("{0}")]
    Api(String),
}

impl AppError {
    pub fn config(message: impl Into<String>) -> AppError {
        AppError::Config(message.into())
    }

    pub fn network(message: impl Into<String>) -> AppError {
        AppError::Network(message.into())
    }

    pub fn api(message: impl Into<String>) -> AppError {
        AppError::Api(message.into())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> AppError {
        if err.is_timeout() {
            AppError::Network("the request timed out".to_string())
        } else if err.is_connect() {
            AppError::Network("the connection could not be established".to_string())
        } else if err.is_decode() {
            AppError::Network("the reply could not be read".to_string())
        } else {
            AppError::Network("the request failed".to_string())
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> AppError {
        AppError::Network("the reply could not be read".to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

fn upper_first(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

pub fn refusal(message: &str) -> String {
    let text = message.to_ascii_lowercase();
    let known = if text.contains("temporary or custom domains") {
        "the mailbox domain is not on the trusted list"
    } else if text.contains("already been claimed") {
        "the trial was already claimed on this device or network"
    } else if text.contains("base email already exists") {
        "the mailbox already holds an account"
    } else if text.contains("sub-addressing") {
        "the mailbox uses an alias the site refuses"
    } else if text.contains("email not verified") {
        "the mailbox has not been verified yet"
    } else if text.contains("invalid email or password") {
        "the sign in was refused"
    } else if text.contains("invalid token") {
        "the code was refused"
    } else if text.contains("unauthorized") {
        "the session was refused"
    } else if text.contains("rate limit") || text.contains("too many") {
        "the server is rate limiting the account"
    } else {
        return short(message);
    };
    upper_first(known)
}

pub const UNKNOWN_REFUSAL: &str = "The server refused the request without a reason";

fn short(_message: &str) -> String {
    UNKNOWN_REFUSAL.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_claimed_trial_is_written_for_a_human() {
        assert_eq!(
            refusal("A 1 GB Free Trial has already been claimed on this device or network."),
            "The trial was already claimed on this device or network"
        );
    }

    #[test]
    fn a_blocked_mailbox_domain_is_named() {
        assert_eq!(
            refusal("Temporary or custom domains are blocked."),
            "The mailbox domain is not on the trusted list"
        );
    }

    #[test]
    fn an_unknown_refusal_never_echoes_the_server_words() {
        let text = refusal("Only 1 concurrent sessions allowed per wallet address");
        assert_eq!(text, UNKNOWN_REFUSAL);
        assert!(!text.contains("concurrent"));
    }

    #[test]
    fn an_empty_refusal_still_reads() {
        assert_eq!(refusal("   "), UNKNOWN_REFUSAL);
    }
}
