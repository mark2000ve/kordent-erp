use kordent_application::{create_organization, rename_organization};

#[test]
fn creates_an_organization_through_the_public_application_api() {
    let organization =
        create_organization("  KORDENT ERP  ").expect("organization should be created");

    assert_eq!(organization.name().as_str(), "KORDENT ERP");
}

#[test]
fn renames_an_organization_through_the_public_application_api() {
    let mut organization = create_organization("KORDENT").expect("organization should be created");
    let id = organization.id();

    rename_organization(&mut organization, "  KORDENT ERP  ")
        .expect("organization should be renamed");

    assert_eq!(organization.id(), id);
    assert_eq!(organization.name().as_str(), "KORDENT ERP");
}
