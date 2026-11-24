use crate::job::store;
use crate::job::Env;
use crate::support::log::format::{gap, lr, ly};

pub async fn run(env: &Env) {
    let lines = match store::read_lines(&env.data) {
        Ok(lines) => lines,
        Err(err) => {
            lr(&err.to_string());
            return;
        }
    };

    if lines.is_empty() {
        lr("File data.txt is empty and holds no email password pairs");
        return;
    }

    let mut missing = 0usize;
    for line in &lines {
        if line.password.is_none() {
            missing += 1;
        }
    }

    let stored = store::read_keys(&env.apikey)
        .map(|keys| keys.len())
        .unwrap_or(0);

    ly(&format!("Account lines in data.txt number {}", lines.len()));
    ly(&format!(
        "Account lines without a password number {}",
        missing
    ));
    ly(&format!("Stored keys in apikey.txt number {}", stored));

    for (index, line) in lines.iter().take(10).enumerate() {
        if line.password.is_some() {
            ly(&format!("Account {} already carries a password", index + 1));
        } else {
            ly(&format!("Account {} still needs a password", index + 1));
        }
    }
    if lines.len() > 10 {
        ly(&format!(
            "Account lines not listed here number {}",
            lines.len() - 10
        ));
    }
    gap();
}
