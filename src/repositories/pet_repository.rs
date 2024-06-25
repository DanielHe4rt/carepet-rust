use std::sync::Arc;

use anyhow::{anyhow, Result};
use charybdis::types::Uuid;
use scylla::{prepared_statement::PreparedStatement, Session};

use crate::model::pet::Pet;

const INSERT_PET_QUERY: &str = "INSERT INTO pets (owner_id,pet_id) VALUES (?, ?)";

const LIST_PET_QUERY: &str = r"
    SELECT
        owner_id,
        pet_id,
        chip_id,
        species,
        breed,
        color,
        gender,
        age,
        weight,
        address,
        name
    FROM
        pets
    WHERE owner_id = ?
    ";

const DEFAULT_PAGE_SIZE: i32 = 5;

pub struct PetRepository {
    session: Arc<Session>,
    insert_pet_query: PreparedStatement,
    list_pet_query: PreparedStatement,
}

impl PetRepository {
    pub async fn new(session: Arc<Session>) -> Self {
        let insert_pet_query = session.prepare(INSERT_PET_QUERY).await.unwrap();
        let mut list_pet_query = session.prepare(LIST_PET_QUERY).await.unwrap();
        list_pet_query.set_page_size(5);

        Self {
            session,
            insert_pet_query,
            list_pet_query
        }
    }

    pub async fn create(&self, pet: Pet) -> Result<()> {
        self.session
            .execute(&self.insert_pet_query, (pet.owner_id, pet.pet_id))
            .await?;

        Ok(())
    }

    pub async fn list_by_owner_id(&mut self, id: Uuid, per_page: i32) -> Result<Vec<Pet>> {

        if per_page != DEFAULT_PAGE_SIZE {
            self.list_pet_query.set_page_size(per_page);
        }

        let result = self.session.execute(&self.list_pet_query, (id,)).await?;
        let pets = result.rows_typed::<Pet>()?.collect::<Result<Vec<_>, _>>()?;

        if pets.len() == 0 {
            return Err(anyhow!("Pet not found"));
        }

        Ok(pets)
    }
}
