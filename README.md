# otp_generator_custom_length

`otp_generator_custom_length` is a Rust library for generating dynamic one-time passwords (OTPs). It provides a simple interface to generate OTP codes of variable lengths.

## Installation

To use `otp_generator_custom_length` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
otp_generator_custom_length = "0.1.0"

```rust
use otp_generator_custom_length::generate_otp;

fn main() {
    let otp_length = 6;
    let otp_code = generate_otp(otp_length);

    println!("Generated OTP code: {}", otp_code);
}
```