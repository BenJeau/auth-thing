mod crypto;
mod error;
mod key;
mod steps;

pub use crypto::CryptoAlgorithm;
pub use error::{Error, Result};

#[derive(Debug, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
pub struct Totp {
    /// Secret key used to generate the OTP, which is zeroized on drop
    secret: String,
    /// Crytographic algorithm used to generate the OTP
    #[zeroize(skip)]
    algorithm: CryptoAlgorithm,
    /// Number of digits of the OTP (from 1 to 9)
    #[zeroize(skip)]
    digits: u8,
    /// Length of a single time step in seconds
    #[zeroize(skip)]
    period: u64,
    /// Number of seconds before the current time that the OTP is valid
    ///
    /// **Note:** most OTP verifiers do not support this
    #[zeroize(skip)]
    offset: u64,
}

pub struct BufferRange {
    start: u64,
    end: u64,
}

impl From<(u64, u64)> for BufferRange {
    fn from(range: (u64, u64)) -> Self {
        Self {
            start: range.0,
            end: range.1,
        }
    }
}

impl Totp {
    pub fn new(
        secret: &str,
        algorithm: CryptoAlgorithm,
        digits: u8,
        period: u64,
        offset: u64,
    ) -> Self {
        Self {
            algorithm,
            secret: secret.to_string(),
            digits,
            period,
            offset,
        }
    }

    pub fn new_with_defaults(secret: &str) -> Self {
        Self::new(secret, CryptoAlgorithm::Sha1, 6, 30, 0)
    }

    pub fn generate_otp_now(&self) -> Result<String> {
        self.generate_otp_at(std::time::UNIX_EPOCH)
    }

    pub fn generate_otp_at(&self, timestamp: impl steps::SystemTimeExt) -> Result<String> {
        let steps = steps::get_number_of_steps(self.offset, self.period, timestamp);
        self.algorithm
            .generate_totp(&self.secret, steps, self.digits)
    }

    pub fn generate_otp_sequence(
        &self,
        timestamp: impl steps::SystemTimeExt,
        buffer: &BufferRange,
    ) -> Result<Vec<String>> {
        let current_step = steps::get_number_of_steps(self.offset, self.period, timestamp);

        if buffer.start > current_step {
            return Err(Error::InvalidBeforeRange(buffer.start, current_step));
        }

        let step_range = (current_step - buffer.start)..=(current_step + buffer.end);

        step_range
            .map(|step| {
                self.algorithm
                    .generate_totp(&self.secret, step, self.digits)
            })
            .collect()
    }

    pub fn validate_otp(
        &self,
        otp: &str,
        timestamp: impl steps::SystemTimeExt,
        buffer: &BufferRange,
    ) -> Result<bool> {
        self.generate_otp_sequence(timestamp, buffer)
            .map(|otps| otps.iter().any(|o| o == otp))
    }

    /// Definition/format: https://github.com/google/google-authenticator/wiki/Key-Uri-Format
    pub fn otpauth_uri(&self, label: &str, issuer: &str) -> String {
        let mut uri = String::from("otpauth://totp/");
        uri.push_str(&urlencoding::encode(label));
        uri.push_str("?secret=");

        let base32_secret = base32::encode(
            base32::Alphabet::Rfc4648 { padding: false },
            self.secret.as_bytes(),
        );

        uri.push_str(&base32_secret);
        uri.push_str("&issuer=");
        uri.push_str(issuer);
        uri.push_str("&algorithm=");
        uri.push_str(&self.algorithm.to_string());
        uri.push_str("&digits=");
        uri.push_str(&self.digits.to_string());
        uri.push_str("&period=");
        uri.push_str(&self.period.to_string());
        uri
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: &str = "12345678901234567890";
    const SEED32: &str = "12345678901234567890123456789012";
    const SEED64: &str = "1234567890123456789012345678901234567890123456789012345678901234";

    struct MockSystemTime(u64);

    impl steps::SystemTimeExt for MockSystemTime {
        fn elapsed(&self) -> std::time::Duration {
            std::time::Duration::from_secs(self.0)
        }
    }

    // https://datatracker.ietf.org/doc/html/rfc6238#appendix-A
    #[test]
    fn test_rfc6238_known_values() {
        let test_times = [
            59u64,
            1111111109,
            1111111111,
            1234567890,
            2000000000,
            20000000000,
        ];
        let expected = [
            ["94287082", "46119246", "90693936"],
            ["07081804", "68084774", "25091201"],
            ["14050471", "67062674", "99943326"],
            ["89005924", "91819424", "93441116"],
            ["69279037", "90698825", "38618901"],
            ["65353130", "77737706", "47863826"],
        ];

        for (time, expected) in test_times.iter().zip(expected.iter()) {
            let steps = time / 30;

            let totp_sha1 = CryptoAlgorithm::Sha1.generate_totp(SEED, steps, 8).unwrap();
            let totp_sha256 = CryptoAlgorithm::Sha256
                .generate_totp(SEED32, steps, 8)
                .unwrap();
            let totp_sha512 = CryptoAlgorithm::Sha512
                .generate_totp(SEED64, steps, 8)
                .unwrap();

            assert_eq!(&totp_sha1, expected[0]);
            assert_eq!(&totp_sha256, expected[1]);
            assert_eq!(&totp_sha512, expected[2]);
        }
    }

    #[test]
    fn test_generate_otp_now() {
        // Gets a period that is greater than the difference of the current time and the Unix epoch,
        // which will make the test deterministic regardless of the current time since the number of
        // steps will always be 0
        let period = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() + 1;
        let secret = "testing_secret";
        let totp = Totp::new(secret, CryptoAlgorithm::Sha1, 8, period, 0);
        let code = totp.generate_otp_now().unwrap();
        assert_eq!(code, "41553593");
    }

    #[test]
    fn test_generate_otp_at() {
        let secret = "testing_secret";
        let totp = Totp::new(secret, CryptoAlgorithm::Sha1, 8, 30, 0);
        let code = totp.generate_otp_at(MockSystemTime(59)).unwrap();
        assert_eq!(code, "61456674");
    }

    #[test]
    fn test_generate_otp_sequence_with_0_before_and_after() {
        let secret = "testing_secret";
        let totp = Totp::new(secret, CryptoAlgorithm::Sha1, 8, 30, 0);
        let sequence = totp
            .generate_otp_sequence(MockSystemTime(59), &(0, 0).into())
            .unwrap();
        assert_eq!(sequence, vec!["61456674"]);
    }

    #[test]
    fn test_generate_otp_sequence_with_1_before_and_0_after() {
        let secret = "testing_secret";
        let totp = Totp::new(secret, CryptoAlgorithm::Sha1, 8, 30, 0);
        let sequence = totp
            .generate_otp_sequence(MockSystemTime(59), &(1, 0).into())
            .unwrap();
        assert_eq!(sequence, vec!["41553593", "61456674"]);
    }

    #[test]
    fn test_generate_otp_sequence_with_0_before_and_1_after() {
        let secret = "testing_secret";
        let totp = Totp::new(secret, CryptoAlgorithm::Sha1, 8, 30, 0);
        let sequence = totp
            .generate_otp_sequence(MockSystemTime(59), &(0, 1).into())
            .unwrap();
        assert_eq!(sequence, vec!["61456674", "49594351"]);
    }

    #[test]
    fn test_generate_otp_sequence_with_before_count_greater_than_number_of_steps_then_returns_error(
    ) {
        let secret = "testing_secret";
        let totp = Totp::new(secret, CryptoAlgorithm::Sha1, 8, 30, 0);
        let result = totp.generate_otp_sequence(MockSystemTime(59), &(2, 0).into());
        assert_eq!(result, Err(Error::InvalidBeforeRange(2, 1)));
    }

    #[test]
    fn test_generate_otp_sequence_with_2_before_and_3_after() {
        let secret = "testing_secret";
        let totp = Totp::new(secret, CryptoAlgorithm::Sha1, 8, 30, 0);
        let sequence = totp
            .generate_otp_sequence(MockSystemTime(89), &(2, 3).into())
            .unwrap();
        assert_eq!(
            sequence,
            vec!["41553593", "61456674", "49594351", "73533374", "19509204", "11802581"]
        );
    }

    #[test]
    fn test_validate_otp_with_correct_otp() {
        let secret = "testing_secret";
        let totp = Totp::new(secret, CryptoAlgorithm::Sha1, 8, 30, 0);
        let result = totp.validate_otp("73533374", MockSystemTime(59), &(1, 2).into());
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_validate_otp_with_incorrect_otp() {
        let secret = "testing_secret";
        let totp = Totp::new(secret, CryptoAlgorithm::Sha1, 8, 30, 0);
        let result = totp.validate_otp("61456675", MockSystemTime(59), &(0, 0).into());
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_otpauth_uri() {
        let secret = "testing_secret";
        let totp = Totp::new(secret, CryptoAlgorithm::Sha1, 8, 30, 0);
        let uri = totp.otpauth_uri("Label wow", "testingOk");
        assert_eq!(
            uri,
            "otpauth://totp/Label%20wow?secret=ORSXG5DJNZTV643FMNZGK5A&issuer=testingOk&algorithm=SHA1&digits=8&period=30"
        );
    }
}
