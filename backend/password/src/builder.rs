use tracing::instrument;

use crate::requirements::{PasswordRequirements, PasswordStrength};

pub struct PasswordRequirementsBuilder(PasswordRequirements);

pub enum PasswordRequirementsBuilderError {
    MinGreaterThanMax,
    SumOfMinRequirementsGreaterThanMinLength,
}

impl PasswordRequirementsBuilder {
    pub fn new() -> Self {
        Self(PasswordRequirements::no_requirements())
    }

    pub fn min(self, min_length: usize) -> Self {
        Self(PasswordRequirements {
            min_length,
            ..self.0
        })
    }

    pub fn max(self, max_length: usize) -> Self {
        Self(PasswordRequirements {
            max_length,
            ..self.0
        })
    }

    pub fn min_lowercase(self, min_lowercase: usize) -> Self {
        Self(PasswordRequirements {
            min_lowercase,
            ..self.0
        })
    }

    pub fn min_uppercase(self, min_uppercase: usize) -> Self {
        Self(PasswordRequirements {
            min_uppercase,
            ..self.0
        })
    }

    pub fn min_number(self, min_number: usize) -> Self {
        Self(PasswordRequirements {
            min_number,
            ..self.0
        })
    }

    pub fn min_special(self, min_special: usize) -> Self {
        Self(PasswordRequirements {
            min_special,
            ..self.0
        })
    }

    pub fn unique(self, unique: bool) -> Self {
        Self(PasswordRequirements { unique, ..self.0 })
    }

    pub fn min_strength(self, min_strength: PasswordStrength) -> Self {
        Self(PasswordRequirements {
            min_strength,
            ..self.0
        })
    }

    #[instrument(skip_all)]
    pub fn build(self) -> Result<PasswordRequirements, PasswordRequirementsBuilderError> {
        if self.0.min_length > self.0.max_length {
            return Err(PasswordRequirementsBuilderError::MinGreaterThanMax);
        }

        if (self.0.min_lowercase + self.0.min_uppercase + self.0.min_number + self.0.min_special)
            > self.0.min_length
        {
            return Err(PasswordRequirementsBuilderError::SumOfMinRequirementsGreaterThanMinLength);
        }

        Ok(self.0)
    }
}
