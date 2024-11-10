#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidDigits(u8),
    InvalidBeforeRange(u64, u64),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDigits(digits) => {
                write!(f, "Invalid number of digits requested: {}", digits)
            }
            Self::InvalidBeforeRange(before, steps) => {
                write!(
                    f,
                    "Invalid range of steps requested creates a negative number of steps: before_range={} steps={}",
                    before, steps
                )
            }
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_invalid_digits_then_error_is_returned() {
        let result = Error::InvalidDigits(10);

        assert_eq!(result.to_string(), "Invalid number of digits requested: 10");
    }

    #[test]
    fn test_given_invalid_before_range_then_error_is_returned() {
        let result = Error::InvalidBeforeRange(10, 20);

        assert_eq!(
            result.to_string(),
            "Invalid range of steps requested creates a negative number of steps: before_range=10 steps=20"
        );
    }
}
