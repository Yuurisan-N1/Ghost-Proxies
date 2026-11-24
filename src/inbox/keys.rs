pub enum Key {
    Digit(u8),
    Erase,
    Resend,
    Skip,
    Stop,
    Idle,
}

pub struct RawMode;

static SAVED: std::sync::Mutex<Option<platform::Saved>> = std::sync::Mutex::new(None);

pub fn restore() {
    if let Ok(mut slot) = SAVED.lock() {
        if let Some(saved) = slot.take() {
            platform::turn_off(saved);
        }
    }
}

impl RawMode {
    pub fn enter() -> Option<RawMode> {
        let saved = platform::turn_on()?;
        if let Ok(mut slot) = SAVED.lock() {
            *slot = Some(saved);
        }
        Some(RawMode)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        restore();
    }
}

pub fn read_key() -> Key {
    let mut one = [0u8; 1];
    match std::io::Read::read(&mut std::io::stdin(), &mut one) {
        Ok(1) => classify(one[0]),
        _ => Key::Idle,
    }
}

fn classify(byte: u8) -> Key {
    match byte {
        b'0'..=b'9' => Key::Digit(byte),
        0x7f | 0x08 => Key::Erase,
        0x12 => Key::Resend,
        0x13 => Key::Skip,
        0x03 => Key::Stop,
        _ => Key::Idle,
    }
}

#[cfg(unix)]
mod platform {
    pub struct Saved(libc::termios);

    pub fn turn_on() -> Option<Saved> {
        let mut term: libc::termios = unsafe { std::mem::zeroed() };
        if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut term) } != 0 {
            return None;
        }
        let saved = Saved(unsafe { std::ptr::read(&term) });
        let mut raw = term;
        raw.c_lflag &= !(libc::ICANON | libc::ECHO);
        raw.c_iflag &= !(libc::IXON | libc::ICRNL);
        raw.c_cc[libc::VMIN] = 1;
        raw.c_cc[libc::VTIME] = 0;
        if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &raw) } != 0 {
            return None;
        }
        Some(saved)
    }

    pub fn turn_off(saved: Saved) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &saved.0);
        }
    }
}

#[cfg(windows)]
mod platform {
    const STD_INPUT_HANDLE: u32 = -10i32 as u32;
    const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
    const ENABLE_LINE_INPUT: u32 = 0x0002;
    const ENABLE_ECHO_INPUT: u32 = 0x0004;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetStdHandle(which: u32) -> *mut core::ffi::c_void;
        fn GetConsoleMode(handle: *mut core::ffi::c_void, mode: *mut u32) -> i32;
        fn SetConsoleMode(handle: *mut core::ffi::c_void, mode: u32) -> i32;
    }

    pub struct Saved(u32);

    pub fn turn_on() -> Option<Saved> {
        unsafe {
            let handle = GetStdHandle(STD_INPUT_HANDLE);
            if handle.is_null() {
                return None;
            }
            let mut mode: u32 = 0;
            if GetConsoleMode(handle, &mut mode) == 0 {
                return None;
            }
            let saved = Saved(mode);
            let raw = mode & !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT);
            if SetConsoleMode(handle, raw) == 0 {
                return None;
            }
            Some(saved)
        }
    }

    pub fn turn_off(saved: Saved) {
        unsafe {
            let handle = GetStdHandle(STD_INPUT_HANDLE);
            let _ = SetConsoleMode(handle, saved.0);
        }
    }
}

#[cfg(not(any(unix, windows)))]
mod platform {
    pub struct Saved;

    pub fn turn_on() -> Option<Saved> {
        None
    }

    pub fn turn_off(_saved: Saved) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_digit_is_read_as_a_digit() {
        assert!(matches!(classify(b'7'), Key::Digit(b'7')));
    }

    #[test]
    fn control_r_asks_for_a_resend() {
        assert!(matches!(classify(0x12), Key::Resend));
    }

    #[test]
    fn control_s_asks_for_a_skip() {
        assert!(matches!(classify(0x13), Key::Skip));
    }

    #[test]
    fn control_c_stops_the_run() {
        assert!(matches!(classify(0x03), Key::Stop));
    }

    #[test]
    fn a_letter_is_ignored() {
        assert!(matches!(classify(b'x'), Key::Idle));
    }
}
