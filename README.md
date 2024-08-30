# SmsAero Rust Api client

[![Crates.io](https://img.shields.io/crates/v/smsaero)](https://crates.io/crates/smsaero)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Library for sending SMS messages using the SmsAero API. Written in Rust.

## Installation:

```bash
cargo install smsaero
```

## Usage example:

Get credentials from account settings page: https://smsaero.ru/cabinet/settings/apikey/

```rust
use chrono::Utc;
use serde_json::Value;
use std::error::Error;
use smsaero::SmsAero;

const SMSAERO_EMAIL: &str = "your email";
const SMSAERO_API_KEY: &str = "your api key";

fn main() -> Result<(), Box<dyn Error>> {
    let client = SmsAero::new(
        SMSAERO_EMAIL.to_string(),
        SMSAERO_API_KEY.to_string(),
        None,
        None,
    );

    match client.send_sms("70000000000", "Hello, world!", None, None) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("SmsAero error: {}", e),
    }

    Ok(())
}
```

## License

```
MIT License
```
