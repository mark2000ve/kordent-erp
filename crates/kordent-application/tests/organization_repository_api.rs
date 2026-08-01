use std::{
    convert::Infallible,
    future::{Future, ready},
};

use kordent_application::{OrganizationRepository, create_organization};
use kordent_core::Organization;

struct TestOrganizationRepository;

impl OrganizationRepository for TestOrganizationRepository {
    type Error = Infallible;

    fn save(
        &self,
        _organization: &Organization,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        ready(Ok(()))
    }
}

#[test]
fn implements_the_public_organization_repository_port() {
    let repository = TestOrganizationRepository;
    let organization = create_organization("KORDENT").expect("organization should be created");

    let future = repository.save(&organization);

    assert_send(&future);
}

fn assert_send<T: Send>(_: &T) {}
