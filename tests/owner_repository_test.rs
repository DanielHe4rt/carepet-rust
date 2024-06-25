use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use care_pet::cli::ServerConfig;
use care_pet::database::new_session_with_keyspace;
use care_pet::model::owner::Owner;
use care_pet::repositories::owner_repository::OwnerRepository;
use uuid::Uuid;


#[tokio::test]
async fn owner_can_be_created() -> Result<()> {
    let session = new_session_with_keyspace(&ServerConfig::default()).await.expect("Connection refused.");
    let session = Arc::new(session);

    let owner = Owner {
        owner_id: Uuid::from_str("03d61583-2f22-450f-8ae1-3c425e0761ad").unwrap(),
        name: "Daniel Reis".to_string(),
        address: "Flamengo St. 1337".to_string(),
    };

    let owner_repository = OwnerRepository::new(session).await;
    owner_repository.create(owner).await?;

    Ok(())
}