use std::time::Duration;

use super::init::{blank, clear_live, emit, live};

pub const G: &str = "\x1b[1;32m";
pub const Y: &str = "\x1b[1;33m";
pub const R: &str = "\x1b[1;31m";

pub fn lg(msg: &str) {
    emit(G, msg);
}

pub fn ly(msg: &str) {
    emit(Y, msg);
}

pub fn lr(msg: &str) {
    emit(R, msg);
}

pub fn gap() {
    blank();
}

pub fn status(msg: &str) {
    live(Y, msg);
}

pub fn clear_status() {
    clear_live();
}

pub fn squeeze(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut sp = false;
    for c in input.chars() {
        if c.is_control() || c == '\u{feff}' {
            if c == '\n' || c == '\r' || c == '\t' {
                sp = true;
            }
            continue;
        }
        if c.is_whitespace() {
            sp = true;
            continue;
        }
        if sp && !out.is_empty() {
            out.push(' ');
        }
        sp = false;
        out.push(c);
    }
    out
}

pub fn clean_text(value: &str, fallback: &str) -> String {
    let mut text = String::with_capacity(value.len());
    for c in value.chars() {
        if "[]|#!@$%^&*()-".contains(c) {
            text.push(' ');
        } else {
            text.push(c);
        }
    }
    let text = squeeze(&text);
    if text.is_empty() {
        fallback.to_string()
    } else {
        text
    }
}

pub fn clock(seconds: u64) -> String {
    let days = seconds / 86_400;
    let rest = seconds % 86_400;
    let hours = rest / 3_600;
    let minutes = (rest % 3_600) / 60;
    let secs = rest % 60;
    if days > 0 {
        format!("{:02}:{:02}:{:02}:{:02}", days, hours, minutes, secs)
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    }
}

pub fn unit_word(count: usize, noun: &str) -> String {
    if count == 1 {
        noun.to_string()
    } else {
        format!("{}s", noun)
    }
}

pub fn mask_email(email: &str) -> String {
    match email.split_once('@') {
        Some((head, domain)) => {
            let keep: String = head.chars().take(2).collect();
            let tail: String = "*".repeat(head.chars().count().saturating_sub(2).max(3));
            format!("{}{}@{}", keep, tail, domain)
        }
        None => "***".to_string(),
    }
}

pub fn mask_key(key: &str) -> String {
    let chars: Vec<char> = key.chars().collect();
    if chars.len() <= 18 {
        return "**".to_string();
    }
    let head: String = chars[..8].iter().collect();
    let tail: String = chars[chars.len() - 10..].iter().collect();
    format!("{}**{}", head, tail)
}

pub async fn countdown(seconds: u64, label: &str) {
    if seconds == 0 {
        return;
    }
    let text = clean_text(label, "Next cycle starts in");
    let mut left = seconds;
    while left > 0 {
        status(&format!("{} {}", text, clock(left)));
        tokio::time::sleep(Duration::from_secs(1)).await;
        left -= 1;
    }
    clear_status();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_mail_is_masked_after_two_letters() {
        assert_eq!(mask_email("driver@example.com"), "dr****@example.com");
    }

    #[test]
    fn a_short_mail_still_hides_the_local_part() {
        assert_eq!(mask_email("ab@example.com"), "ab***@example.com");
    }

    #[test]
    fn a_key_keeps_the_head_and_the_tail() {
        let masked = mask_key("gp_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        assert!(masked.starts_with("gp_aaaa"));
        assert!(masked.contains("**"));
        assert!(masked.ends_with("aaaaaaaaaa"));
    }

    #[test]
    fn a_short_key_is_fully_hidden() {
        assert_eq!(mask_key("gp_short"), "**");
    }

    #[test]
    fn the_clock_reads_as_hours_minutes_seconds() {
        assert_eq!(clock(0), "00:00:00");
        assert_eq!(clock(3_725), "01:02:05");
        assert_eq!(clock(90_061), "01:01:01:01");
    }

    #[test]
    fn squeeze_collapses_control_whitespace() {
        assert_eq!(squeeze("a\n\nb\tc"), "a b c");
    }

    #[test]
    fn clean_text_replaces_brackets_and_pipes() {
        assert_eq!(clean_text("[a]|b", "fallback"), "a b");
    }

    #[tokio::test]
    async fn the_countdown_ticks_once_a_second() {
        let start = std::time::Instant::now();
        countdown(2, "Next cycle starts in").await;
        assert!(start.elapsed() >= std::time::Duration::from_millis(1_900));
    }

    #[test]
    fn a_single_account_reads_as_one_account() {
        assert_eq!(unit_word(1, "account"), "account");
        assert_eq!(unit_word(2, "account"), "accounts");
    }

    #[test]
    fn clean_text_falls_back_when_empty() {
        assert_eq!(clean_text("   ", "fallback"), "fallback");
    }
}
