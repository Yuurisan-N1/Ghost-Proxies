use crate::inbox::keys::{read_key, Key, RawMode};
use crate::support::log::format::{clear_status, mask_email, status};

pub enum Answer {
    Code(String),
    Resend,
    Skip,
    Stop,
}

pub fn ask_otp(email: &str) -> Answer {
    let shown = mask_email(email);
    let mode = match RawMode::enter() {
        Some(mode) => mode,
        None => return Answer::Code(piped(&shown)),
    };
    let mut typed = String::new();
    loop {
        status(&line(&shown, typed.len()));
        match read_key() {
            Key::Digit(byte) => {
                if typed.len() < 6 {
                    typed.push(byte as char);
                }
                if typed.len() == 6 {
                    clear_status();
                    drop(mode);
                    return Answer::Code(typed);
                }
            }
            Key::Erase => {
                typed.pop();
            }
            Key::Resend => {
                clear_status();
                drop(mode);
                return Answer::Resend;
            }
            Key::Skip => {
                clear_status();
                drop(mode);
                return Answer::Skip;
            }
            Key::Stop => {
                clear_status();
                drop(mode);
                return Answer::Stop;
            }
            Key::Idle => {}
        }
    }
}

fn piped(shown: &str) -> String {
    status(&line(shown, 0));
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let mut raw = String::new();
    if std::io::BufRead::read_line(&mut std::io::stdin().lock(), &mut raw).is_err() {
        clear_status();
        return String::new();
    }
    clear_status();
    raw.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn line(shown: &str, filled: usize) -> String {
    let stars = "*".repeat(filled);
    if filled == 0 {
        format!("Verification code for {}", shown)
    } else {
        format!("Verification code for {} {}", shown, stars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_prompt_names_the_masked_mailbox() {
        let text = line("la***@gmail.com", 0);
        assert!(text.contains("la***@gmail.com"));
    }

    #[test]
    fn the_typed_digits_show_as_stars_only() {
        let text = line("la***@gmail.com", 3);
        assert!(text.ends_with("***"));
        assert!(!text.contains('7'));
    }
}
