use crate::gateway::route;
use crate::net::egress::Proxy;
use crate::security::fingerprint;
use crate::support::device::Device;
use crate::support::log::format::{gap, lg, lr, mask_email, mask_key};

pub fn run() {
    ly_open();
    let mut passed = 0usize;
    let mut failed = 0usize;

    check(
        &mut passed,
        &mut failed,
        fingerprint::password().chars().count() >= 16,
        "The generated password carries at least sixteen characters",
    );

    let device = Device::fresh("us");
    check(
        &mut passed,
        &mut failed,
        device.agent.starts_with("Mozilla/5.0"),
        "The user agent reads as a real browser",
    );
    check(
        &mut passed,
        &mut failed,
        device.fingerprint.len() == 32,
        "The fingerprint is thirty two characters long",
    );

    match Proxy::parse("http://user__cr.sg:pass@192.0.2.10:8080") {
        Ok(found) => {
            let next = found.with_country("de");
            check(
                &mut passed,
                &mut failed,
                next.user.as_deref() == Some("user__cr.de"),
                "The proxy country mark is swapped on the copy",
            );
        }
        Err(_) => {
            lr("The proxy line in the self test could not be parsed");
            failed += 1;
        }
    }

    let path = route::route_for("us");
    check(
        &mut passed,
        &mut failed,
        route::tunnel(&path, "gp_selftest").is_ok(),
        "The gateway tunnel is built from the route",
    );
    check(
        &mut passed,
        &mut failed,
        route::describe(&path) == "http://res-us:**@gate.ghostproxies.net:8080",
        "The gateway route is described for the operator",
    );

    check(
        &mut passed,
        &mut failed,
        mask_email("tester@example.com") == "te****@example.com",
        "The mailbox label is masked after two letters",
    );
    check(
        &mut passed,
        &mut failed,
        mask_key("gp_0123456789abcdef0123456789abcdef0123456789abcdef") == "gp_01234**6789abcdef",
        "The key is masked in the middle for the log",
    );

    gap();
    if failed > 0 {
        lr(&format!("{} checks failed on this build", failed));
        return;
    }
    lg(&format!("All {} checks passed on this run", passed));
}

fn ly_open() {
    crate::support::log::format::ly("The self test runs without touching the network");
}

fn check(passed: &mut usize, failed: &mut usize, flag: bool, label: &str) {
    if flag {
        *passed += 1;
        lg(&format!("The check passed for {}", label));
    } else {
        *failed += 1;
        lr(&format!("The check failed for {}", label));
    }
}
