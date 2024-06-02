use std::sync::Arc;

use anyhow::Result;
use care_pet::cli::ServerConfig;
use care_pet::database::new_session_with_keyspace;
use care_pet::repositories::owner_repository::OwnerRepository;

use crate::common::setup;

mod common;

#[tokio::test]
async fn owner_can_be_created() -> Result<()> {
    let session = new_session_with_keyspace(&ServerConfig::default()).await.expect("Connection refused.");
    let session = Arc::new(session);

    let (owner) = setup(Arc::clone(&session)).await;

    let owner_repository = OwnerRepository::new(session).await;

    owner_repository.create(owner).await?;

    Ok(())
}