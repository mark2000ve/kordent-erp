use kordent_core::{Organization, OrganizationName, OrganizationNameError};

pub fn create_organization(name: impl Into<String>) -> Result<Organization, OrganizationNameError> {
    OrganizationName::new(name).map(Organization::new)
}

#[cfg(test)]
mod tests {
    use super::create_organization;
    use kordent_core::OrganizationNameError;

    #[test]
    fn creates_an_organization_from_a_name() {
        let organization =
            create_organization("  KORDENT ERP  ").expect("organization should be created");

        assert_eq!(organization.name().as_str(), "KORDENT ERP");
    }

    #[test]
    fn rejects_an_empty_organization_name() {
        let result = create_organization("   ");

        assert_eq!(result, Err(OrganizationNameError::Empty));
    }
}
