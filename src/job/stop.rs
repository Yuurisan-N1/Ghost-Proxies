use crate::inbox::keys;
use crate::support::log::format::{clear_status, gap, lr};

pub fn now() -> ! {
    keys::restore();
    clear_status();
    gap();
    lr("Script stopped by user");
    std::process::exit(0);
}
