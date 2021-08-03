#![allow(unused_imports)]
use crate::otp::OTP;
use sha1::Sha1;
use hmac::{Hmac, Mac, NewMac};
use data_encoding::BASE32;
use std::time::{SystemTime, UNIX_EPOCH};


type HmacSha1 = Hmac<Sha1>;


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct HOTP {
	mac: HmacSha1,
}