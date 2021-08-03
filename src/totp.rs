use crate::otp::OTP;
use sha1::Sha1;
use hmac::{Hmac, Mac, NewMac};
use data_encoding::BASE32;
use std::time::{SystemTime, UNIX_EPOCH};


type HmacSha1 = Hmac<Sha1>;


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct TOTP {
	time_step: u64,
	start_time: u64,
	digit_count: u64,
	mac: HmacSha1,
}

impl TOTP {
	pub fn new(time_step: u64, start_time: u64, digit_count: u64, mac: HmacSha1) -> Self {
		Self {
			time_step, start_time, digit_count, mac
		}
	}

	pub fn from_secret(secret: &[u8]) -> Self {
		let secret = BASE32.decode(secret).expect("Invalid secret!");
		let mac = HmacSha1::new_from_slice(&secret)
			.expect("HMAC can take key of any size");

		Self::new(30, 0, 6, mac)
	}
}


impl OTP for TOTP {
	fn generate(&mut self) -> u64 {
		let now = SystemTime::now();
		let since_epoch = now
			.duration_since(UNIX_EPOCH)
			.expect("Time went backwards!")
			.as_secs();

		let timestamp = (since_epoch - self.start_time) / self.time_step;
		let seed = timestamp.to_be_bytes();

		self.mac.update(&seed);
		let hash = self.mac
			.finalize_reset()
			.into_bytes();

		let mut truncated: u64 = 0;
		let offset = (hash[19] & 0x0F) as usize;

		for i in offset..offset + 4 {
			truncated <<= 8;
			truncated |= (hash[i] & 0xFF) as u64;
		}

		truncated &= 0x7FFFFFFF;
		truncated %= (10 as u64).pow(self.digit_count as u32);
		truncated
	}
}