use std::{error::Error, fmt, str::FromStr};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    id: OrganizationId,
    name: OrganizationName,
}

impl Organization {
    #[must_use]
    pub fn new(name: OrganizationName) -> Self {
        Self {
            id: OrganizationId::new(),
            name,
        }
    }

    #[must_use]
    pub const fn from_parts(id: OrganizationId, name: OrganizationName) -> Self {
        Self { id, name }
    }

    #[must_use]
    pub const fn id(&self) -> OrganizationId {
        self.id
    }

    #[must_use]
    pub const fn name(&self) -> &OrganizationName {
        &self.name
    }

    pub fn rename(&mut self, name: OrganizationName) {
        self.name = name;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrganizationId(Uuid);

impl OrganizationId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for OrganizationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for OrganizationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for OrganizationId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value).map(Self)
    }
}

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
    use super::{Organization, OrganizationId, OrganizationName, OrganizationNameError};
    use uuid::Uuid;

    #[test]
    fn creates_an_organization_with_a_generated_identifier() {
        let name = OrganizationName::new("KORDENT ERP").expect("name should be valid");
        let organization = Organization::new(name);

        assert_eq!(organization.name().as_str(), "KORDENT ERP");
        assert_eq!(organization.id().as_uuid().get_version_num(), 7);
    }

    #[test]
    fn restores_an_organization_from_existing_parts() {
        let id = OrganizationId::from_uuid(Uuid::from_u128(42));
        let name = OrganizationName::new("KORDENT").expect("name should be valid");
        let organization = Organization::from_parts(id, name);

        assert_eq!(organization.id(), id);
        assert_eq!(organization.name().as_str(), "KORDENT");
    }

    #[test]
    fn renames_an_organization_without_changing_its_identifier() {
        let original_name = OrganizationName::new("KORDENT").expect("name should be valid");
        let mut organization = Organization::new(original_name);
        let original_id = organization.id();

        let new_name = OrganizationName::new("KORDENT ERP").expect("name should be valid");
        organization.rename(new_name);

        assert_eq!(organization.id(), original_id);
        assert_eq!(organization.name().as_str(), "KORDENT ERP");
    }

    #[test]
    fn generates_an_organization_id_that_round_trips_through_text() {
        let id = OrganizationId::new();
        let encoded = id.to_string();
        let decoded = encoded
            .parse::<OrganizationId>()
            .expect("generated identifier should be valid");

        assert_eq!(decoded, id);
    }

    #[test]
    fn wraps_an_existing_uuid() {
        let uuid = Uuid::from_u128(42);
        let id = OrganizationId::from_uuid(uuid);

        assert_eq!(id.as_uuid(), &uuid);
    }

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
