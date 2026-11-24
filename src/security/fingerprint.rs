use rand::rngs::OsRng;
use rand::Rng;

use crate::support::consts::app::{PASSWORD_LEN, PASSWORD_TAIL};

const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const DIGIT: &[u8] = b"23456789";

pub fn password() -> String {
    let mut rng = OsRng;
    let mut out = String::with_capacity(PASSWORD_LEN + PASSWORD_TAIL.len());
    out.push(UPPER[rng.gen_range(0..UPPER.len())] as char);
    out.push(LOWER[rng.gen_range(0..LOWER.len())] as char);
    out.push(DIGIT[rng.gen_range(0..DIGIT.len())] as char);
    for _ in 3..PASSWORD_LEN {
        let pool: &[u8] = match rng.gen_range(0..3) {
            0 => LOWER,
            1 => UPPER,
            _ => DIGIT,
        };
        out.push(pool[rng.gen_range(0..pool.len())] as char);
    }
    out.push_str(PASSWORD_TAIL);
    out
}

pub fn display_name() -> String {
    let mut rng = OsRng;
    const FIRST: &[&str] = &[
        "alex", "jordan", "casey", "riley", "morgan", "taylor", "avery", "quinn", "harper",
        "rowan", "sawyer", "dakota",
    ];
    const LAST: &[&str] = &[
        "reed", "hayes", "brooks", "price", "walsh", "foster", "graham", "keller", "marsh", "vance",
    ];
    format!(
        "{} {}",
        FIRST[rng.gen_range(0..FIRST.len())],
        LAST[rng.gen_range(0..LAST.len())]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_password_carries_every_class() {
        let secret = password();
        assert!(secret.chars().any(|c| c.is_ascii_uppercase()));
        assert!(secret.chars().any(|c| c.is_ascii_lowercase()));
        assert!(secret.chars().any(|c| c.is_ascii_digit()));
        assert!(secret.ends_with(PASSWORD_TAIL));
        assert_eq!(secret.chars().count(), PASSWORD_LEN + PASSWORD_TAIL.len());
    }

    #[test]
    fn two_passwords_differ() {
        let mut seen = std::collections::HashSet::new();
        for _ in 0..50 {
            seen.insert(password());
        }
        assert_eq!(seen.len(), 50);
    }

    #[test]
    fn a_password_has_no_ambiguous_letters() {
        for _ in 0..50 {
            let secret = password();
            assert!(!secret.contains('l'));
            assert!(!secret.contains('I'));
            assert!(!secret.contains('O'));
            assert!(!secret.contains('0'));
            assert!(!secret.contains('1'));
        }
    }

    #[test]
    fn a_name_has_two_parts() {
        let name = display_name();
        assert_eq!(name.split(' ').count(), 2);
    }
}
