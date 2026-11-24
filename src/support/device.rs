use rand::rngs::OsRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::support::consts::app::FINGERPRINT_LEN;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub agent: String,
    pub brands: String,
    pub platform: String,
    pub mobile: String,
    pub language: String,
    pub timezone: String,
    pub width: u32,
    pub height: u32,
    pub scale: f64,
    pub fingerprint: String,
}

#[derive(Clone, Copy, PartialEq)]
enum Family {
    Chrome,
    Edge,
    Firefox,
}

#[derive(Clone, Copy, PartialEq)]
enum Os {
    Windows,
    Mac,
    Linux,
}

const CHROME_VERSIONS: &[u32] = &[126, 127, 128, 129, 130, 131, 132];
const EDGE_VERSIONS: &[u32] = &[126, 127, 128, 129, 130, 131];
const FIREFOX_VERSIONS: &[u32] = &[128, 129, 130, 131, 132, 133];

const SCREENS: &[(u32, u32, f64)] = &[
    (1920, 1080, 1.0),
    (1536, 864, 1.25),
    (1440, 900, 1.0),
    (1366, 768, 1.0),
    (1600, 900, 1.5),
    (2560, 1440, 1.25),
    (1280, 720, 1.0),
    (1680, 1050, 1.0),
];

const LOCALES: &[(&str, &str, &str)] = &[
    ("us", "en-US,en;q=0.9", "America/New_York"),
    ("us", "en-US,en;q=0.9", "America/Chicago"),
    ("us", "en-US,en;q=0.9", "America/Los_Angeles"),
    ("gb", "en-GB,en;q=0.9", "Europe/London"),
    ("ca", "en-CA,en;q=0.9", "America/Toronto"),
    ("au", "en-AU,en;q=0.9", "Australia/Sydney"),
    ("de", "de-DE,de;q=0.9,en;q=0.8", "Europe/Berlin"),
    ("nl", "nl-NL,nl;q=0.9,en;q=0.8", "Europe/Amsterdam"),
    ("fr", "fr-FR,fr;q=0.9,en;q=0.8", "Europe/Paris"),
    ("sg", "en-SG,en;q=0.9", "Asia/Singapore"),
];

impl Device {
    pub fn fresh(country: &str) -> Device {
        let mut rng = OsRng;
        let family = match rng.gen_range(0..3) {
            0 => Family::Chrome,
            1 => Family::Edge,
            _ => Family::Firefox,
        };
        let os = match rng.gen_range(0..3) {
            0 => Os::Windows,
            1 => Os::Mac,
            _ => Os::Linux,
        };
        let version = match family {
            Family::Chrome => CHROME_VERSIONS[rng.gen_range(0..CHROME_VERSIONS.len())],
            Family::Edge => EDGE_VERSIONS[rng.gen_range(0..EDGE_VERSIONS.len())],
            Family::Firefox => FIREFOX_VERSIONS[rng.gen_range(0..FIREFOX_VERSIONS.len())],
        };
        let pool: Vec<&(&str, &str, &str)> =
            LOCALES.iter().filter(|row| row.0 == country).collect();
        let pick = if pool.is_empty() {
            LOCALES[rng.gen_range(0..LOCALES.len())]
        } else {
            *pool[rng.gen_range(0..pool.len())]
        };
        let (width, height, scale) = SCREENS[rng.gen_range(0..SCREENS.len())];

        Device {
            agent: agent_of(family, os, version),
            brands: brands_of(family, version),
            platform: platform_of(family, os),
            mobile: mobile_of(family),
            language: pick.1.to_string(),
            timezone: pick.2.to_string(),
            width,
            height,
            scale,
            fingerprint: fingerprint(&mut rng),
        }
    }

    #[cfg(test)]
    pub fn plain() -> Device {
        Device {
            agent: agent_of(Family::Chrome, Os::Windows, 131),
            brands: brands_of(Family::Chrome, 131),
            platform: platform_of(Family::Chrome, Os::Windows),
            mobile: mobile_of(Family::Chrome),
            language: "en-US,en;q=0.9".to_string(),
            timezone: "America/New_York".to_string(),
            width: 1920,
            height: 1080,
            scale: 1.0,
            fingerprint: "00000000000000000000000000000000".to_string(),
        }
    }
}

fn agent_of(family: Family, os: Os, version: u32) -> String {
    match (family, os) {
        (Family::Firefox, Os::Windows) => format!(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:{}.0) Gecko/20100101 Firefox/{}.0",
            version, version
        ),
        (Family::Firefox, Os::Mac) => format!(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:{}.0) Gecko/20100101 Firefox/{}.0",
            version, version
        ),
        (Family::Firefox, Os::Linux) => format!(
            "Mozilla/5.0 (X11; Linux x86_64; rv:{}.0) Gecko/20100101 Firefox/{}.0",
            version, version
        ),
        (Family::Chrome, Os::Windows) => format!(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
            version
        ),
        (Family::Chrome, Os::Mac) => format!(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
            version
        ),
        (Family::Chrome, Os::Linux) => format!(
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
            version
        ),
        (Family::Edge, Os::Windows) => format!(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36 Edg/{}.0.0.0",
            version, version
        ),
        (Family::Edge, Os::Mac) => format!(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36 Edg/{}.0.0.0",
            version, version
        ),
        (Family::Edge, Os::Linux) => format!(
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36 Edg/{}.0.0.0",
            version, version
        ),
    }
}

fn brands_of(family: Family, version: u32) -> String {
    match family {
        Family::Chrome => format!(
            "\"Google Chrome\";v=\"{}\", \"Chromium\";v=\"{}\", \"Not_A Brand\";v=\"24\"",
            version, version
        ),
        Family::Edge => format!(
            "\"Microsoft Edge\";v=\"{}\", \"Chromium\";v=\"{}\", \"Not_A Brand\";v=\"24\"",
            version, version
        ),
        Family::Firefox => format!("\"Firefox\";v=\"{}\", \"Not_A Brand\";v=\"24\"", version),
    }
}

fn platform_of(family: Family, os: Os) -> String {
    if family == Family::Firefox {
        return String::new();
    }
    match os {
        Os::Windows => "\"Windows\"".to_string(),
        Os::Mac => "\"macOS\"".to_string(),
        Os::Linux => "\"Linux\"".to_string(),
    }
}

fn mobile_of(family: Family) -> String {
    if family == Family::Firefox {
        return String::new();
    }
    "?0".to_string()
}

fn fingerprint(rng: &mut OsRng) -> String {
    const HEX: &[u8] = b"0123456789abcdef";
    let mut out = String::with_capacity(FINGERPRINT_LEN);
    for _ in 0..FINGERPRINT_LEN {
        out.push(HEX[rng.gen_range(0..HEX.len())] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_device_looks_like_a_real_browser() {
        let device = Device::fresh("us");
        assert!(device.agent.starts_with("Mozilla/5.0"));
        assert!(device.width >= 1280);
        assert_eq!(device.fingerprint.len(), FINGERPRINT_LEN);
    }

    #[test]
    fn two_accounts_never_share_a_profile() {
        let mut agents = std::collections::HashSet::new();
        let mut prints = std::collections::HashSet::new();
        for _ in 0..40 {
            let device = Device::fresh("us");
            agents.insert(device.agent);
            prints.insert(device.fingerprint);
        }
        assert!(agents.len() > 5);
        assert_eq!(prints.len(), 40);
    }

    #[test]
    fn the_brand_version_matches_the_agent_version() {
        for _ in 0..40 {
            let device = Device::fresh("us");
            let number = device
                .brands
                .split("v=\"")
                .nth(1)
                .and_then(|row| row.split('"').next())
                .unwrap_or_default()
                .to_string();
            assert!(!number.is_empty());
            assert!(
                device.agent.contains(&format!("/{}.", number))
                    || device.agent.contains(&format!("rv:{}.0", number))
            );
        }
    }

    #[test]
    fn firefox_carries_no_client_hints() {
        for _ in 0..40 {
            let device = Device::fresh("us");
            if device.agent.contains("Firefox/") {
                assert!(device.platform.is_empty());
                assert!(device.mobile.is_empty());
                assert!(device.brands.contains("Firefox"));
            }
        }
    }

    #[test]
    fn the_country_picks_its_own_locale() {
        let device = Device::fresh("de");
        assert!(device.language.starts_with("de"));
        assert_eq!(device.timezone, "Europe/Berlin");
    }

    #[test]
    fn the_fingerprint_is_lowercase_hex() {
        let device = Device::fresh("us");
        assert!(device
            .fingerprint
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
    }
}
