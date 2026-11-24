use std::path::PathBuf;

use crate::cli::{enroll, probe, selftest};
use crate::job::loopstep;
use crate::job::Env;
use crate::support::banner::print_banner;
use crate::support::cfg::schema::Config;
use crate::support::consts::app::{APIKEY_FILE, CONFIG_FILE, DATA_FILE, PROXY_FILE, STATE_FILE};
use crate::support::log::format::{gap, lr};
use crate::support::log::init::arm;

pub async fn run() {
    arm();
    print_banner();

    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let config = match Config::load(&root.join(CONFIG_FILE)) {
        Ok(config) => config,
        Err(err) => {
            lr(&err.to_string());
            return;
        }
    };

    let env = Env {
        data: root.join(DATA_FILE),
        apikey: root.join(APIKEY_FILE),
        proxy: root.join(PROXY_FILE),
        state: root.join(STATE_FILE),
        config,
    };

    let command = std::env::args().nth(1).unwrap_or_default();
    match command.as_str() {
        "" | "loop" => loopstep::forever(&env).await,
        "once" | "run" => {
            loopstep::once(&env, 1).await;
        }
        "enroll" => enroll::run(&env).await,
        "probe" => probe::run(&env).await,
        "selftest" => selftest::run(),
        other => {
            lr(&format!(
                "The mode name {} is not one this bot knows",
                other
            ));
            gap();
            lr("Use run, loop, enroll, probe or selftest");
        }
    }
}
