mod otp;


pub struct Totp {
	time_step: u64,
	start_time: u64,
	digit_count: u64,
}

impl Totp {
	pub fn new(time_step: u64, start_time: u64, digit_count: u64) -> Self {
		Self {
			time_step, start_time, digit_count
		}
	}
}

impl Default for Totp {
	fn default() -> Self {
		Self {
			time_step: 30,
			start_time: 0,
			digit_count: 6
		}
	}
}

impl Totp {

}