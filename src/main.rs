use std::env;
use xotp::{OTP, TOTP};

fn main() {
	let secret = env::var("OTP_SECRET")
		.expect("Environment variable `OTP_SECRET` not found!");
	let secret = secret.as_bytes();

	let mut otp = TOTP::from_secret(secret);
	assert_eq!(otp.generate(), otp.generate());
	println!("{:?}", otp.generate());
}