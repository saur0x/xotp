pub trait Otp {
    fn generate(&mut self) -> u64;
}