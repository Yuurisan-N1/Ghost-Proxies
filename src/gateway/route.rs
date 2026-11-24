use crate::net::gate;
use crate::support::consts::net::{COUNTRIES, GATE_ROUTE};
use crate::support::error::AppResult;

pub fn route_for(country: &str) -> String {
    if COUNTRIES.contains(&country) {
        format!("{}-{}", GATE_ROUTE, country)
    } else {
        format!("{}-{}", GATE_ROUTE, COUNTRIES[0])
    }
}

pub fn tunnel(route: &str, key: &str) -> AppResult<String> {
    gate::build(route, key)
}

pub fn describe(route: &str) -> String {
    gate::describe(route)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_known_country_keeps_its_route() {
        assert_eq!(route_for("de"), "res-de");
    }

    #[test]
    fn an_unknown_country_falls_back_to_the_first() {
        assert_eq!(route_for("zz"), "res-us");
    }

    #[test]
    fn the_tunnel_carries_the_key() {
        let url = tunnel("res-us", "gp_abc").unwrap();
        assert!(url.contains("gp_abc"));
    }
}
