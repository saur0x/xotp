mod totp;
mod otp;

use std::env;

use otp::Otp;
use totp::Totp;

fn main() {
	let secret = env::var("OTP_SECRET")
		.expect("Environment variable `OTP_SECRET` not found!");
	let secret = secret.as_bytes();

	let mut otp = Totp::from_secret(secret);
	assert_eq!(otp.generate(), otp.generate());
	println!("{:?}", otp.generate());
}