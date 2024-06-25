use std::sync::Arc;

use anyhow::{anyhow, Result};
use scylla::{prepared_statement::PreparedStatement, Session};
use uuid::Uuid;

use crate::model::owner::Owner;

const INSERT_OWNER_QUERY: &str = r"INSERT INTO owners (owner_id, name, address) VALUES (?, ?, ?)";

const FIND_OWNER_BY_PARTITION: &str = r"SELECT owner_id, name, address FROM owners WHERE owner_id = ?";

pub struct OwnerRepository {
    session: Arc<Session>,
    insert_owner_query: PreparedStatement,
    find_owner_by_partition: PreparedStatement,
}

impl OwnerRepository {
    pub async fn new(session: Arc<Session>) -> Self {
        let insert_owner_query = session.prepare(INSERT_OWNER_QUERY).await.unwrap();
        let find_owner_by_partition = session.prepare(FIND_OWNER_BY_PARTITION).await.unwrap();
        Self {
            session,
            insert_owner_query,
            find_owner_by_partition,
        }
    }

    pub async fn create(&self, owner: Owner) -> Result<()> {
        self.session
            .execute(
                &self.insert_owner_query,
                (owner.owner_id, owner.name, owner.address),
            )
            .await?;

        Ok(())
    }

    pub async fn find(&self, id: Uuid) -> Result<Owner> {
        let result = self
            .session
            .execute(&self.find_owner_by_partition, (id,))
            .await?
            .rows_typed::<Owner>()?;

        if let Some(owner) = result.into_iter().next().transpose()? {
            return Ok(owner);
        }

        Err(anyhow!("Owner not found"))
    }
}
