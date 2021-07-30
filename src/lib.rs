mod totp;
// pub use totp::Totp;


#[cfg(test)]
mod tests {
    #[test]
    fn totp_new() {
        let t = totp::Totp::new(0, 0, 0);

    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
