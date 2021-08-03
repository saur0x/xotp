#[allow(non_camel_case_types)]
pub trait OTP {
    fn generate(&mut self) -> u64;
}