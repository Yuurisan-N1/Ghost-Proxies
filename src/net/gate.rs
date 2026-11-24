use crate::support::consts::net::{GATE_HOST, GATE_PORT};
use crate::support::error::{AppError, AppResult};

pub fn describe(route: &str) -> String {
    format!("http://{}:**@{}:{}", route.trim(), GATE_HOST, GATE_PORT)
}

pub fn build(route: &str, key: &str) -> AppResult<String> {
    if key.trim().is_empty() {
        return Err(AppError::config("apikey.txt holds no key for the gateway"));
    }
    if route.trim().is_empty() {
        return Err(AppError::config("the gateway route is empty"));
    }
    Ok(format!(
        "http://{}:{}@{}:{}",
        route.trim(),
        key.trim(),
        GATE_HOST,
        GATE_PORT
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_route_and_a_key_build_a_tunnel() {
        let url = build("res-us", "gp_test").unwrap();
        assert_eq!(url, "http://res-us:gp_test@gate.ghostproxies.net:8080");
    }

    #[test]
    fn an_empty_key_is_refused() {
        assert!(build("res-us", "  ").is_err());
    }

    #[test]
    fn an_empty_route_is_refused() {
        assert!(build(" ", "gp_test").is_err());
    }
}
