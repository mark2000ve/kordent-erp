use kordent_core::{Organization, OrganizationName, OrganizationNameError};

pub fn create_organization(name: impl Into<String>) -> Result<Organization, OrganizationNameError> {
    OrganizationName::new(name).map(Organization::new)
}

pub fn rename_organization(
    organization: &mut Organization,
    name: impl Into<String>,
) -> Result<(), OrganizationNameError> {
    let name = OrganizationName::new(name)?;
    organization.rename(name);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{create_organization, rename_organization};
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

    #[test]
    fn renames_an_organization_from_a_name() {
        let mut organization =
            create_organization("KORDENT").expect("organization should be created");
        let id = organization.id();

        rename_organization(&mut organization, "  KORDENT ERP  ")
            .expect("organization should be renamed");

        assert_eq!(organization.id(), id);
        assert_eq!(organization.name().as_str(), "KORDENT ERP");
    }

    #[test]
    fn keeps_the_existing_name_when_rename_input_is_empty() {
        let mut organization =
            create_organization("KORDENT").expect("organization should be created");

        let result = rename_organization(&mut organization, "   ");

        assert_eq!(result, Err(OrganizationNameError::Empty));
        assert_eq!(organization.name().as_str(), "KORDENT");
    }
}
