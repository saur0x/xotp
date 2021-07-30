pub trait Otp {
    pub fn generate(&self) -> u64;
}