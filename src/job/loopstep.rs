use std::collections::BTreeMap;

use crate::job::onboard::{self, Outcome};
use crate::job::store::{self, Record};
use crate::job::Env;
use crate::support::log::format::{countdown, gap, lg, lr, ly, unit_word};

pub async fn once(env: &Env, cycle: u64) -> usize {
    let mut lines = match store::read_lines(&env.data) {
        Ok(lines) => lines,
        Err(err) => {
            lr(&err.to_string());
            return 0;
        }
    };
    if lines.is_empty() {
        lr("File data.txt is empty and holds no email password pairs");
        return 0;
    }

    match store::fill_passwords(&env.data) {
        Ok(filled) if filled > 0 => {
            ly(&format!(
                "Generated passwords were written back to {} {}",
                filled,
                unit_word(filled, "line")
            ));
            lines = match store::read_lines(&env.data) {
                Ok(lines) => lines,
                Err(err) => {
                    lr(&err.to_string());
                    return 0;
                }
            };
        }
        Ok(_) => {}
        Err(err) => {
            lr(&err.to_string());
            return 0;
        }
    }

    let mut state: BTreeMap<String, Record> = match store::load_state(&env.state) {
        Ok(state) => state,
        Err(err) => {
            lr(&err.to_string());
            BTreeMap::new()
        }
    };

    let mut stored = 0usize;

    for (index, line) in lines.iter().enumerate() {
        if index > 0 {
            gap();
        }
        let slot = index + 1;
        ly(&format!("Account {} started the cycle", slot));

        if state
            .get(&line.email)
            .map(|record| !record.apikey.trim().is_empty())
            .unwrap_or(false)
        {
            ly(&format!(
                "Account {} already holds a key so it was skipped",
                slot
            ));
            continue;
        }

        match onboard::run(env, &mut state, line, index).await {
            Ok(Outcome::Stored) => stored += 1,
            Ok(Outcome::Skipped(reason)) => {
                ly(&format!("Account {} was skipped because {}", slot, reason))
            }
            Err(err) => lr(&format!("Account {} failed because {}", slot, err)),
        }
    }

    gap();
    lg(&format!(
        "All accounts processed for cycle number {}",
        cycle
    ));
    stored
}

pub async fn forever(env: &Env) {
    let mut number = 1u64;
    loop {
        ly(&format!("Starting automation cycle number {}", number));
        once(env, number).await;
        countdown(env.config.sleep_seconds(), "Next cycle starts in").await;
        number += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_loop_module_links() {
        let _ = super::once;
        let _ = super::forever;
    }
}
