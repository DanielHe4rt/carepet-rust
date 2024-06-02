use std::str::FromStr;
use std::sync::Arc;

use scylla::query::Query;
use scylla::Session;
use uuid::Uuid;

use care_pet::model::owner::Owner;

pub async fn setup(session: Arc<Session>) -> (Owner) {
    let owner = setup_owner(session).await;

    (owner)
}


async fn setup_owner(session: Arc<Session>) -> Owner {
    let owner = Owner {
        owner_id: Uuid::from_str("03d61583-2f22-450f-8ae1-3c425e0761ad").unwrap(),
        name: "Daniel Reis".to_string(),
        address: "Flamengo St. 1337".to_string(),
    };

    let insert_owner_query = "INSERT INTO owners (owner_id, name, address) VALUES (?,?,?)";
    let query = Query::new(insert_owner_query);

    session.query(query, (owner.clone().owner_id, owner.clone().name, owner.clone().address))
        .await
        .expect("Failed to insert owner");

    owner
}

pub async fn teardown(session: Arc<Session>) {}