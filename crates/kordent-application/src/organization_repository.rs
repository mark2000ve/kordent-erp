use std::future::Future;

use kordent_core::Organization;

pub trait OrganizationRepository {
    type Error;

    fn save(
        &self,
        organization: &Organization,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
