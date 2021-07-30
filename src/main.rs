use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};
use std::time::{SystemTime, UNIX_EPOCH};
// use xotp::Totp;


type HmacSha256 = Hmac<Sha256>;

fn main() {
	// Create HMAC-SHA256 instance which implements `Mac` trait
	let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
		.expect("HMAC can take key of any size");
	mac.update(b"input message");

	// `result` has type `Output` which is a thin wrapper around array of
	// bytes for providing constant time equality check
	let result = mac.finalize();
	// To get underlying array use `into_bytes` method, but be careful, since
	// incorrect use of the code value may permit timing attacks which defeat
	// the security provided by the `Output`
	let code_bytes = result.into_bytes();
	println!("{:?}", code_bytes);

	let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
    .expect("HMAC can take key of any size");

	mac.update(b"input message");

	// `verify` will return `Ok(())` if code is correct, `Err(MacError)` otherwise
	mac.verify(&code_bytes).unwrap();

	let now = SystemTime::now();
	let since_epoch = now
		.duration_since(UNIX_EPOCH)
		.expect("Time went backwards!");

	println!("{:?}", since_epoch.as_secs());
}