use kordent_application::create_organization;

#[test]
fn creates_an_organization_through_the_public_application_api() {
    let organization =
        create_organization("  KORDENT ERP  ").expect("organization should be created");

    assert_eq!(organization.name().as_str(), "KORDENT ERP");
}
