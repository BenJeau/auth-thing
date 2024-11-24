use tracing::instrument;

use crate::{
    crypto::verify_password,
    requirements::{PasswordError, PasswordRequirements, PasswordStrength, RequiredActual},
};

pub(crate) struct PasswordValidator<'a> {
    pub password: &'a str,
    pub requirements: &'a PasswordRequirements,
    pub previous_hashed_passwords: &'a [String],
}

impl<'a> PasswordValidator<'a> {
    pub(crate) fn validate(&self) -> Vec<PasswordError> {
        self.validate_length()
            .into_iter()
            .chain(self.validate_character_requirements())
            .chain(self.validate_strength())
            .chain(self.validate_reused())
            .collect()
    }

    #[instrument(skip_all)]
    fn validate_length(&self) -> Vec<PasswordError> {
        let mut errors = Vec::with_capacity(2);

        if self.password.len() < self.requirements.min_length {
            errors.push(PasswordError::TooShort(RequiredActual {
                required: self.requirements.min_length,
                actual: self.password.len(),
            }));
        }
        if self.password.len() > self.requirements.max_length {
            errors.push(PasswordError::TooLong(RequiredActual {
                required: self.requirements.max_length,
                actual: self.password.len(),
            }));
        }

        errors
    }

    #[instrument(skip_all)]
    fn validate_character_requirements(&self) -> Vec<PasswordError> {
        let mut errors = Vec::with_capacity(4);
        let mut lowercase_count = 0;
        let mut uppercase_count = 0;
        let mut number_count = 0;
        let mut special_count = 0;

        for c in self.password.chars() {
            if c.is_lowercase() {
                lowercase_count += 1;
            } else if c.is_uppercase() {
                uppercase_count += 1;
            } else if c.is_digit(10) {
                number_count += 1;
            } else {
                special_count += 1;
            }
        }

        if lowercase_count < self.requirements.min_lowercase {
            errors.push(PasswordError::MissingLowercase(RequiredActual {
                required: self.requirements.min_lowercase,
                actual: lowercase_count,
            }));
        }
        if uppercase_count < self.requirements.min_uppercase {
            errors.push(PasswordError::MissingUppercase(RequiredActual {
                required: self.requirements.min_uppercase,
                actual: uppercase_count,
            }));
        }
        if number_count < self.requirements.min_number {
            errors.push(PasswordError::MissingNumber(RequiredActual {
                required: self.requirements.min_number,
                actual: number_count,
            }));
        }
        if special_count < self.requirements.min_special {
            errors.push(PasswordError::MissingSpecial(RequiredActual {
                required: self.requirements.min_special,
                actual: special_count,
            }));
        }

        errors
    }

    #[instrument(skip_all)]
    fn validate_strength(&self) -> Vec<PasswordError> {
        // TODO: Should take into account user inputs in calculation
        let entropy = zxcvbn::zxcvbn(self.password, &[]);
        let strength = PasswordStrength::from(entropy.score());

        if strength < self.requirements.min_strength {
            vec![PasswordError::DoesNotMeetMinimumStrength(RequiredActual {
                required: self.requirements.min_strength,
                actual: strength,
            })]
        } else {
            vec![]
        }
    }

    #[instrument(skip_all)]
    fn validate_reused(&self) -> Vec<PasswordError> {
        if self.previous_hashed_passwords.is_empty() || !self.requirements.unique {
            return vec![];
        }

        if self
            .previous_hashed_passwords
            .into_iter()
            .any(|previous_hashed_password| {
                verify_password(self.password, &previous_hashed_password).unwrap()
            })
        {
            vec![PasswordError::Reused]
        } else {
            vec![]
        }
    }
}
