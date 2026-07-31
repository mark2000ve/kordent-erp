use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrganizationName(String);

impl OrganizationName {
    pub fn new(value: impl Into<String>) -> Result<Self, OrganizationNameError> {
        let value = value.into();
        let value = value.trim();

        if value.is_empty() {
            return Err(OrganizationNameError::Empty);
        }

        Ok(Self(value.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for OrganizationName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganizationNameError {
    Empty,
}

impl fmt::Display for OrganizationNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("organization name cannot be empty"),
        }
    }
}

impl Error for OrganizationNameError {}

#[cfg(test)]
mod tests {
    use super::{OrganizationName, OrganizationNameError};

    #[test]
    fn creates_an_organization_name() {
        let name = OrganizationName::new("KORDENT").expect("name should be valid");

        assert_eq!(name.as_str(), "KORDENT");
        assert_eq!(name.to_string(), "KORDENT");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let name = OrganizationName::new("  KORDENT ERP  ").expect("name should be valid");

        assert_eq!(name.as_str(), "KORDENT ERP");
    }

    #[test]
    fn rejects_an_empty_organization_name() {
        let result = OrganizationName::new("   ");

        assert_eq!(result, Err(OrganizationNameError::Empty));
    }
}
