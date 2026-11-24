pub mod loopstep;
pub mod onboard;
pub mod stop;
pub mod store;

use std::path::PathBuf;

use crate::support::cfg::schema::Config;

#[derive(Debug, Clone)]
pub struct Env {
    pub data: PathBuf,
    pub apikey: PathBuf,
    pub proxy: PathBuf,
    pub state: PathBuf,
    pub config: Config,
}
