use serde::Serialize;
use tracing::{instrument, warn};
use utoipa::ToSchema;

use crate::validator::PasswordValidator;

#[derive(Serialize, ToSchema)]
pub struct PasswordRequirements {
    pub(crate) min_length: usize,
    pub(crate) max_length: usize,
    pub(crate) min_lowercase: usize,
    pub(crate) min_uppercase: usize,
    pub(crate) min_number: usize,
    pub(crate) min_special: usize,
    pub(crate) unique: bool,
    pub(crate) min_strength: PasswordStrength,
}

impl PasswordRequirements {
    pub(crate) fn no_requirements() -> Self {
        Self {
            min_length: usize::MIN,
            max_length: usize::MAX,
            min_lowercase: 0,
            min_uppercase: 0,
            min_number: 0,
            min_special: 0,
            unique: false,
            min_strength: PasswordStrength::Weak,
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Serialize, ToSchema)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
}

impl From<&str> for PasswordStrength {
    fn from(strength: &str) -> Self {
        match strength {
            "weak" => PasswordStrength::Weak,
            "medium" => PasswordStrength::Medium,
            "strong" => PasswordStrength::Strong,
            _ => {
                warn!("Invalid password strength: {}", strength);
                PasswordStrength::Weak
            }
        }
    }
}

impl From<zxcvbn::Score> for PasswordStrength {
    fn from(score: zxcvbn::Score) -> Self {
        match score {
            zxcvbn::Score::Three => PasswordStrength::Medium,
            zxcvbn::Score::Four => PasswordStrength::Strong,
            _ => PasswordStrength::Weak,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct RequiredActual<T> {
    pub required: T,
    pub actual: T,
}

#[derive(PartialEq, Eq, Debug)]
pub enum PasswordError {
    TooShort(RequiredActual<usize>),
    TooLong(RequiredActual<usize>),
    MissingLowercase(RequiredActual<usize>),
    MissingUppercase(RequiredActual<usize>),
    MissingNumber(RequiredActual<usize>),
    MissingSpecial(RequiredActual<usize>),
    DoesNotMeetMinimumStrength(RequiredActual<PasswordStrength>),
    Reused,
}

impl PasswordRequirements {
    #[instrument(skip_all)]
    pub fn validate(
        &self,
        password: &str,
        previous_hashed_passwords: &[String],
    ) -> Vec<PasswordError> {
        PasswordValidator {
            password,
            requirements: self,
            previous_hashed_passwords,
        }
        .validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hash_password;

    #[test]
    fn test_given_no_requirements_when_validating_empty_string_then_returns_no_errors() {
        let no_requirements = PasswordRequirements::no_requirements();

        let errors = no_requirements.validate("", &[]);

        assert!(errors.is_empty());
    }

    // min length
    #[test]
    fn test_given_min_length_8_when_validating_password_shorter_than_8_then_returns_error() {
        let min_length_8_requirements = PasswordRequirements {
            min_length: 8,
            ..PasswordRequirements::no_requirements()
        };

        let errors = min_length_8_requirements.validate("short", &[]);

        assert_eq!(
            errors,
            vec![PasswordError::TooShort(RequiredActual {
                required: 8,
                actual: 5,
            })]
        );
    }

    #[test]
    fn test_given_min_length_8_when_validating_password_longer_than_8_then_returns_no_errors() {
        let min_length_8_requirements = PasswordRequirements {
            min_length: 8,
            ..PasswordRequirements::no_requirements()
        };

        let errors = min_length_8_requirements.validate("more_than_8", &[]);

        assert!(errors.is_empty());
    }

    // max length
    #[test]
    fn test_given_max_length_10_when_validating_password_longer_than_10_then_returns_error() {
        let max_length_10_requirements = PasswordRequirements {
            max_length: 10,
            ..PasswordRequirements::no_requirements()
        };

        let errors = max_length_10_requirements.validate("11character", &[]);

        assert_eq!(
            errors,
            vec![PasswordError::TooLong(RequiredActual {
                required: 10,
                actual: 11,
            })]
        );
    }

    #[test]
    fn test_given_max_length_10_when_validating_password_shorter_than_10_then_returns_no_errors() {
        let max_length_10_requirements = PasswordRequirements {
            max_length: 10,
            ..PasswordRequirements::no_requirements()
        };

        let errors = max_length_10_requirements.validate("short", &[]);

        assert!(errors.is_empty());
    }

    // lowercase
    #[test]
    fn test_given_requires_lowercase_when_validating_password_without_lowercase_then_returns_error()
    {
        let lowercase_requirements = PasswordRequirements {
            min_lowercase: 1,
            ..PasswordRequirements::no_requirements()
        };

        let errors = lowercase_requirements.validate("ALLCAPS", &[]);

        assert_eq!(
            errors,
            vec![PasswordError::MissingLowercase(RequiredActual {
                required: 1,
                actual: 0,
            })]
        );
    }

    #[test]
    fn test_given_requires_lowercase_when_validating_password_with_lowercase_then_returns_no_errors(
    ) {
        let lowercase_requirements = PasswordRequirements {
            min_lowercase: 1,
            ..PasswordRequirements::no_requirements()
        };

        let errors = lowercase_requirements.validate("ONLY_ONE_LOWERCASEa", &[]);

        assert!(errors.is_empty());
    }

    // uppercase
    #[test]
    fn test_given_requires_uppercase_when_validating_password_without_uppercase_then_returns_error()
    {
        let uppercase_requirements = PasswordRequirements {
            min_uppercase: 1,
            ..PasswordRequirements::no_requirements()
        };

        let errors = uppercase_requirements.validate("only_lowercase", &[]);

        assert_eq!(
            errors,
            vec![PasswordError::MissingUppercase(RequiredActual {
                required: 1,
                actual: 0,
            })]
        );
    }

    #[test]
    fn test_given_requires_uppercase_when_validating_password_with_uppercase_then_returns_no_errors(
    ) {
        let uppercase_requirements = PasswordRequirements {
            min_uppercase: 1,
            ..PasswordRequirements::no_requirements()
        };

        let errors = uppercase_requirements.validate("only_one_uppercaseA", &[]);

        assert!(errors.is_empty());
    }

    // number
    #[test]
    fn test_given_requires_number_when_validating_password_without_number_then_returns_error() {
        let number_requirements = PasswordRequirements {
            min_number: 1,
            ..PasswordRequirements::no_requirements()
        };

        let errors = number_requirements.validate("onlyletters", &[]);

        assert_eq!(
            errors,
            vec![PasswordError::MissingNumber(RequiredActual {
                required: 1,
                actual: 0,
            })]
        );
    }

    #[test]
    fn test_given_requires_number_when_validating_password_with_number_then_returns_no_errors() {
        let number_requirements = PasswordRequirements {
            min_number: 1,
            ..PasswordRequirements::no_requirements()
        };

        let errors = number_requirements.validate("onlyonenumber1", &[]);

        assert!(errors.is_empty());
    }

    // special
    #[test]
    fn test_given_requires_special_when_validating_password_without_special_then_returns_error() {
        let special_requirements = PasswordRequirements {
            min_special: 1,
            ..PasswordRequirements::no_requirements()
        };

        let errors = special_requirements.validate("onlyletters", &[]);

        assert_eq!(
            errors,
            vec![PasswordError::MissingSpecial(RequiredActual {
                required: 1,
                actual: 0,
            })]
        );
    }

    #[test]
    fn test_given_requires_special_when_validating_password_with_special_then_returns_no_errors() {
        let special_requirements = PasswordRequirements {
            min_special: 1,
            ..PasswordRequirements::no_requirements()
        };

        let errors = special_requirements.validate("only_one_special!", &[]);

        assert!(errors.is_empty());
    }

    // reused
    #[test]
    fn test_given_no_previous_hashed_passwords_when_validating_password_then_returns_no_errors() {
        let unique_requirements = PasswordRequirements {
            unique: true,
            ..PasswordRequirements::no_requirements()
        };

        let errors = unique_requirements.validate("password", &[]);

        assert!(errors.is_empty());
    }

    #[test]
    fn test_given_previous_hashed_passwords_with_no_requirements_when_validating_password_then_returns_no_errors(
    ) {
        let no_requirements = PasswordRequirements::no_requirements();

        let errors = no_requirements.validate("password", &[hash_password("password").unwrap()]);

        assert!(errors.is_empty());
    }

    #[test]
    fn test_given_previous_hashed_password_when_validating_password_then_returns_error() {
        let unique_requirements = PasswordRequirements {
            unique: true,
            ..PasswordRequirements::no_requirements()
        };

        let errors =
            unique_requirements.validate("password", &[hash_password("password").unwrap()]);

        assert_eq!(errors, vec![PasswordError::Reused]);
    }

    #[test]
    fn test_given_previous_hashed_password_when_validating_different_password_then_returns_no_errors(
    ) {
        let unique_requirements = PasswordRequirements {
            unique: true,
            ..PasswordRequirements::no_requirements()
        };

        let errors =
            unique_requirements.validate("different", &[hash_password("password").unwrap()]);

        assert!(errors.is_empty());
    }

    // strength
    #[test]
    fn test_given_min_strength_weak_when_validating_common_password_then_returns_no_errors() {
        let min_strength_weak_requirements = PasswordRequirements {
            min_strength: PasswordStrength::Weak,
            ..PasswordRequirements::no_requirements()
        };

        let errors = min_strength_weak_requirements.validate("password", &[]);

        assert!(errors.is_empty());
    }

    #[test]
    fn test_given_min_strength_medium_when_validating_common_password_then_returns_error() {
        let min_strength_medium_requirements = PasswordRequirements {
            min_strength: PasswordStrength::Medium,
            ..PasswordRequirements::no_requirements()
        };

        let errors = min_strength_medium_requirements.validate("password", &[]);

        assert_eq!(
            errors,
            vec![PasswordError::DoesNotMeetMinimumStrength(RequiredActual {
                required: PasswordStrength::Medium,
                actual: PasswordStrength::Weak,
            })]
        );
    }

    #[test]
    fn test_given_min_strength_medium_when_validating_strong_password_then_returns_no_errors() {
        let min_strength_medium_requirements = PasswordRequirements {
            min_strength: PasswordStrength::Medium,
            ..PasswordRequirements::no_requirements()
        };

        let errors = min_strength_medium_requirements.validate("strong_password", &[]);

        assert!(errors.is_empty());
    }

    #[test]
    fn test_given_min_strength_strong_when_validating_common_password_then_returns_error() {
        let min_strength_strong_requirements = PasswordRequirements {
            min_strength: PasswordStrength::Strong,
            ..PasswordRequirements::no_requirements()
        };

        let errors = min_strength_strong_requirements.validate("password123", &[]);

        assert_eq!(
            errors,
            vec![PasswordError::DoesNotMeetMinimumStrength(RequiredActual {
                required: PasswordStrength::Strong,
                actual: PasswordStrength::Weak,
            })]
        );
    }

    #[test]
    fn test_given_min_strength_strong_when_validating_very_strong_password_then_returns_no_errors()
    {
        let min_strength_strong_requirements = PasswordRequirements {
            min_strength: PasswordStrength::Strong,
            ..PasswordRequirements::no_requirements()
        };

        let errors = min_strength_strong_requirements.validate("very_strong_password", &[]);

        assert!(errors.is_empty());
    }

    // all together
    #[test]
    fn test_given_all_requirements_when_validating_invalid_password_then_returns_errors() {
        let all_requirements = PasswordRequirements {
            min_length: 8,
            max_length: 10,
            min_lowercase: 1,
            min_uppercase: 1,
            min_number: 1,
            min_special: 1,
            unique: true,
            min_strength: PasswordStrength::Medium,
        };

        let errors = all_requirements.validate("", &[hash_password("").unwrap()]);

        assert_eq!(errors.len(), 7);
        assert!(errors.contains(&PasswordError::TooShort(RequiredActual {
            required: 8,
            actual: 0,
        })));
        assert!(
            errors.contains(&PasswordError::MissingLowercase(RequiredActual {
                required: 1,
                actual: 0,
            }))
        );
        assert!(
            errors.contains(&PasswordError::MissingUppercase(RequiredActual {
                required: 1,
                actual: 0,
            }))
        );
        assert!(
            errors.contains(&PasswordError::MissingNumber(RequiredActual {
                required: 1,
                actual: 0,
            }))
        );
        assert!(
            errors.contains(&PasswordError::MissingSpecial(RequiredActual {
                required: 1,
                actual: 0,
            }))
        );
        assert!(
            errors.contains(&PasswordError::DoesNotMeetMinimumStrength(RequiredActual {
                required: PasswordStrength::Medium,
                actual: PasswordStrength::Weak,
            }))
        );
        assert!(errors.contains(&PasswordError::Reused));
    }

    #[test]
    fn test_given_all_requirements_when_validating_valid_password_then_returns_no_errors() {
        let all_requirements = PasswordRequirements {
            min_length: 8,
            max_length: 10,
            min_lowercase: 1,
            min_uppercase: 1,
            min_number: 1,
            min_special: 1,
            unique: true,
            min_strength: PasswordStrength::Medium,
        };

        let errors = all_requirements.validate("Valid123!", &[hash_password("other").unwrap()]);

        assert!(errors.is_empty());
    }
}
