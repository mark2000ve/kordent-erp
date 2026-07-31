use kordent_core::{Organization, OrganizationName};

#[test]
fn creates_and_renames_an_organization_through_the_public_api() {
    let original_name =
        OrganizationName::new("KORDENT").expect("organization name should be valid");
    let mut organization = Organization::new(original_name);
    let original_id = organization.id();

    let new_name = OrganizationName::new("KORDENT ERP").expect("organization name should be valid");
    organization.rename(new_name);

    assert_eq!(organization.id(), original_id);
    assert_eq!(organization.name().as_str(), "KORDENT ERP");
}
