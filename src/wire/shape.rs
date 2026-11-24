use serde_json::{json, Value};

use crate::support::device::Device;

pub fn check_signup(email: &str, device: &Device) -> Value {
    json!({
        "email": email,
        "phoneNumber": "",
        "deviceFingerprint": device.fingerprint,
        "acceptedTerms": true,
    })
}

pub fn signup(email: &str, password: &str, name: &str, device: &Device) -> Value {
    json!({
        "email": email,
        "password": password,
        "name": name,
        "firstName": name,
        "lastName": "user",
        "deviceFingerprint": device.fingerprint,
        "acceptedTerms": true,
    })
}

pub fn signin(email: &str, password: &str) -> Value {
    json!({
        "email": email,
        "password": password,
    })
}

pub fn send_verify(email: &str) -> Value {
    json!({ "email": email })
}

pub fn verify_otp(email: &str, otp: &str) -> Value {
    json!({
        "email": email,
        "otp": otp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_verify_body_names_the_otp_field() {
        let body = verify_otp("a@example.com", "123456");
        assert_eq!(body.get("otp").and_then(Value::as_str), Some("123456"));
        assert!(body.get("token").is_none());
    }

    #[test]
    fn the_signup_body_carries_the_device() {
        let device = Device::plain();
        let body = signup("a@example.com", "pw", "name", &device);
        assert_eq!(
            body.get("deviceFingerprint").and_then(Value::as_str),
            Some(device.fingerprint.as_str())
        );
        assert_eq!(
            body.get("acceptedTerms").and_then(Value::as_bool),
            Some(true)
        );
    }

    #[test]
    fn the_check_signup_body_has_no_phone() {
        let device = Device::plain();
        let body = check_signup("a@example.com", &device);
        assert_eq!(body.get("phoneNumber").and_then(Value::as_str), Some(""));
    }
}
