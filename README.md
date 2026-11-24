<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=Ghost%20Proxies%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Account%20Login%20%7C%20OTP%20Verify%20%7C%20API%20Key%20Collector&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Gmail+%7C+Outlook+Account+Login;OTP+Verification+%7C+Resend+Support;API+Key+Collector+%7C+apikey.txt;Per-Account+Device+Profile+%7C+Country+Rotation;Enroll+Mode+%7C+Probe+%7C+Proxy+Support"/>

<p>
  <img alt="rust" src="https://img.shields.io/badge/Rust-2021-f74c00?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-Ghost%20Proxies-111111"/>
  <img alt="build" src="https://github.com/Yuurisan-N1/Ghost-Proxies/actions/workflows/build.yml/badge.svg"/>
  <img alt="proxy" src="https://img.shields.io/badge/Proxy-Supported-111111"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>Ghost Proxies Bot</b> is an account automation client for Ghost Proxies.<br/>
  It reads Gmail or Outlook accounts from <code>data.txt</code>, signs every account in, verifies new accounts with a six-digit OTP from the inbox, and collects the API key the account already holds into <code>apikey.txt</code>. Each account runs with its own browser device profile and an optional proxy line, so the site sees a distinct identity for every row.<br/>
  Built and distributed by <b>Yuurisandesu</b>.
</p>

</div>

---

## Table of Contents

- [Requirements](#requirements)
- [Installation](#installation)
- [Configuration](#configuration)
- [Running the Bot](#running-the-bot)
- [Run Modes](#run-modes)
- [Features](#features)
- [File Structure](#file-structure)
- [Disclaimer](#disclaimer)

---

## Requirements

- Git
- A supported platform: Linux x86_64, Windows x86_64, or macOS Apple Silicon

No compiler or runtime is needed. The bot ships as a prebuilt binary.

---

## Installation

**Clone the repository:**

```bash
git clone https://github.com/Yuurisan-N1/Ghost-Proxies.git
cd Ghost-Proxies
```

**Download the binary for your platform from the latest release:**

https://github.com/Yuurisan-N1/Ghost-Proxies/releases/latest

| File | Platform |
|---|---|
| `ghostproxies-bot-linux-x86_64` | Linux x86_64 |
| `ghostproxies-bot-windows-x86_64.exe` | Windows x86_64 |
| `ghostproxies-bot-macos-aarch64` | macOS Apple Silicon |

Place the downloaded binary in the same folder as `data.txt`, `proxy.txt`, and `config.json`.

**Linux / macOS — mark executable:**

```bash
chmod +x ghostproxies-bot-linux-x86_64
```

---

## Configuration

### 1. Accounts (data.txt)

Fill `data.txt` with one account per line in `email|password` format. Only Gmail and Outlook addresses are accepted by the site. Blank lines and lines starting with `#` are ignored:

```
first@gmail.com|yourpassword
second@outlook.com|anotherpassword
```

A line without a password is also valid:

```
first@gmail.com|
```

The bot generates a random password on the first run and writes it back into the same line so it is reused on every later run.

### 2. Proxy (proxy.txt)

Fill `proxy.txt` with proxies, one per line (optional, leave empty to connect directly):

```
http://user:pass@host:port
socks5://user:pass@host:port
```

Proxies are paired to accounts by index. The country is taken from the proxy line. A line carrying `__cr.<cc>` rotates that country code across all accounts in turn. Proxy credentials are masked in the log.

> The site limits the free trial to one account per device and network. A residential proxy per account is what makes each row go through.

### 3. Bot Settings (config.json)

`sleep_seconds` controls how many seconds the bot waits between cycles in loop mode. The value is clamped between 60 and 86400 seconds. The default is 3600 (one hour).

---

## Running the Bot

Make `run.sh` executable first (Linux / macOS):

```bash
chmod +x run.sh
```

Then choose a mode:

```bash
./run.sh run            # one pass over data.txt, foreground
./run.sh loop           # repeating passes with sleep_seconds between them, foreground
./run.sh enroll         # same as run, alias for the enroll flow
./run.sh bg             # loop in background, logs to ghostproxies-bot.log
./run.sh bgprobe        # probe in background
./run.sh screen         # loop in a detached screen session
./run.sh stop           # stop the background or screen session
./run.sh probe          # check the site and every proxy without writing anything
./run.sh selftest       # offline checks, no network
```

For background mode, tail the log anytime:

```bash
tail -f ghostproxies-bot.log
```

For screen mode, attach anytime:

```bash
screen -r ghostproxies-bot
```

Press `Ctrl+C` once to stop the bot cleanly when running in foreground.

### Manual run

```bash
# Linux / macOS
./ghostproxies-bot-linux-x86_64

# Windows
.\ghostproxies-bot-windows-x86_64.exe
```

### OTP prompt

When an account is new or unverified, the bot sends a code to the inbox and waits at the terminal:

```
Verification code for fi***st@gmail.com:
```

Type the six digits from the inbox. The code is sent as soon as the sixth digit is typed, no Enter needed.

| Key | Action |
|---|---|
| Six digits | Submit the code |
| Ctrl+R | Request a fresh code when the mail has not arrived |
| Ctrl+S | Skip this account and move to the next row |
| Ctrl+C | Stop the run |

---

## Run Modes

| Mode | Command | Description |
|---|---|---|
| run | `./run.sh run` | One pass over data.txt |
| loop | `./run.sh loop` | Repeating passes with sleep between them |
| enroll | `./run.sh enroll` | Same as run |
| bg | `./run.sh bg` | Loop in background with log file |
| bgprobe | `./run.sh bgprobe` | Probe in background |
| screen | `./run.sh screen` | Loop in a detached screen session |
| probe | `./run.sh probe` | Check site and proxies without writing anything |
| selftest | `./run.sh selftest` | Offline checks, no network |
| stop | `./run.sh stop` | Stop background or screen session |

---

## Features

### Account Login
Every account in `data.txt` is signed in with its email and password. Already signed-in accounts with a cached device identity reuse it on each run.

### New Account Registration
When an account does not exist yet on the site, the bot checks whether the free trial is still available on the current egress and registers the account automatically before proceeding to verification.

### OTP Verification
New and unverified accounts require a six-digit code from the inbox. The bot prompts for the code at the terminal, sends it as soon as the sixth digit is typed, and verifies the account. A fresh code can be requested with Ctrl+R if the mail has not arrived.

### API Key Collection
After sign-in the bot reads the default API key the account already holds and appends it to `apikey.txt`, one key per line. The key is logged in masked form. The bot never creates a new key.

### Per-Account Device Profile
Every row in `data.txt` gets its own browser device profile: user agent, client hints, screen resolution, locale, timezone, and a 32-character hex fingerprint. Nothing is shared between rows. The profile is generated once and cached in `session.json` so it stays stable across runs.

### Country Rotation
The country for each account is taken from the proxy line. A proxy line carrying `__cr.<cc>` rotates that country code across all accounts in turn so each row presents a different location.

### Password Generation
A row without a password in `data.txt` gets a random one generated on the first run. The password is written back into the same line so it is reused on every later run without any manual step.

### Probe Mode
Probe checks the site host and reports whether the free trial gate is open on the current egress for every proxy in `proxy.txt`. Nothing is written to any file during probe.

### Proxy Support
Proxies are loaded from `proxy.txt` and assigned to accounts by position. Running without proxies is fully supported.

### Auto Countdown
After all accounts complete a cycle in loop mode the bot shows a live `HH:MM:SS` countdown until the next cycle starts.

---

## File Structure

```text
Ghost-Proxies/
├── .github/
│   └── workflows/
│       └── build.yml                    # CI: 3-platform matrix build
├── assets/
│   ├── icon.ico                         # Windows binary icon
│   └── icon.png                         # App icon
├── src/
│   ├── auth/
│   │   ├── create.rs                    # Sign-in, sign-up, and trial gate (distributed separately)
│   │   ├── mod.rs
│   │   └── verify.rs                    # OTP send and confirm
│   ├── cli/
│   │   ├── dispatch.rs                  # CLI mode selector
│   │   ├── enroll.rs                    # Enroll mode runner
│   │   ├── mod.rs
│   │   ├── probe.rs                     # Probe mode: site and proxy check
│   │   └── selftest.rs                  # Offline self-test checks
│   ├── gateway/
│   │   ├── mod.rs
│   │   └── route.rs                     # Trial gate route resolver
│   ├── inbox/
│   │   ├── keys.rs                      # API key list fetch
│   │   ├── mod.rs
│   │   └── prompt.rs                    # Interactive OTP prompt with Ctrl+R, Ctrl+S, Ctrl+C
│   ├── job/
│   │   ├── loopstep.rs                  # Cycle runner: once and forever loop
│   │   ├── mod.rs
│   │   ├── onboard.rs                   # Per-account sign-in, verify, key collect flow
│   │   ├── stop.rs                      # Clean stop handler
│   │   └── store.rs                     # data.txt reader, session cache, apikey.txt writer (distributed separately)
│   ├── keys/
│   │   ├── list.rs                      # API key fetch and default key selector
│   │   └── mod.rs
│   ├── net/
│   │   ├── client.rs                    # HTTP client with per-account cookie jar
│   │   ├── egress.rs                    # Proxy loader, picker, and country resolver (distributed separately)
│   │   ├── gate.rs                      # Trial gate HTTP request
│   │   └── mod.rs
│   ├── security/
│   │   ├── fingerprint.rs               # 32-char device fingerprint and password generator
│   │   ├── mod.rs
│   │   └── signup_check.rs              # Signup eligibility check
│   ├── support/
│   │   ├── banner.rs                    # ASCII banner and terminal title
│   │   ├── cfg/
│   │   │   ├── mod.rs
│   │   │   └── schema.rs                # Config schema and sleep setting
│   │   ├── consts/
│   │   │   ├── app.rs                   # File name constants
│   │   │   ├── mod.rs
│   │   │   └── net.rs                   # Base URL and country list (distributed separately)
│   │   ├── device.rs                    # Browser device profile builder
│   │   ├── error.rs                     # Error types and result alias
│   │   ├── log/
│   │   │   ├── format.rs                # Colored log helpers and countdown
│   │   │   ├── init.rs                  # Signal handler setup
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── wire/
│   │   ├── mod.rs
│   │   └── shape.rs                     # JSON response parsers
│   └── main.rs                          # Entry point and mode dispatch
├── .gitignore
├── build.rs                             # Build script: embeds icon into Windows binary
├── Cargo.lock
├── Cargo.toml                           # Project manifest and dependencies
├── config.json                          # Sleep duration setting
├── data.txt                             # Accounts, email|password one per line
├── apikey.txt                           # Collected API keys, one per line (auto-generated)
├── HOWTO.txt                            # Quick start guide
├── LICENSE                              # GNU AGPL v3.0
├── Makefile                             # make targets: build, release, run, start, clean, size
├── proxy.txt                            # Proxy list (optional)
├── README.md                            # This file
├── run.sh                               # Run helper: run, loop, enroll, bg, bgprobe, screen, probe, selftest, stop
└── session.json                         # Device profile and key cache per account (auto-generated)
```

---

## Disclaimer

This tool is built for educational and technical exploration purposes. Use it wisely and at your own responsibility.

---

<div align="center">
<img width="100%" alt="footer" src="https://capsule-render.vercel.app/api?type=waving&height=120&section=footer"/>
</div>