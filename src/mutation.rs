use crate::query::Dog;
use async_graphql::{context::Context, Object};
use sqlx::PgPool;
use crate::broker::SimpleBroker;
use crate::subscription::{DogChanged, MutationType};
use tokio::sync::broadcast::Sender;
pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_dog(&self, ctx: &Context<'_>, name: String, age: i32) -> Result<i32, String> {
        let db = match ctx.data::<PgPool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        let res = match sqlx::query_as::<_, Dog>(
            "INSERT INTO dogs (NAME, AGE) VALUES ($1, $2) RETURNING id, name, age",
        )
        .bind(name)
        .bind(age)
        .fetch_one(db)
        .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

	SimpleBroker::publish(DogChanged {
		mutation_type: MutationType::Created,
		id: res.id,
	});

	Ok(res.id)	
    }

    async fn update_dog(&self, ctx: &Context<'_>, 
	#[graphql(desc = "New name value to update to")]
	name: Option<String>, 
	#[graphql(desc = "New age value to update to")]
	age: Option<i32>, 
	#[graphql(desc = "(REQUIRED) The ID of the record to update")]
	id: i32) -> Result<i32, String> {
        let db = match ctx.data::<PgPool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        let res = match sqlx::query_as::<_, Dog>(
            "UPDATE dogs SET 
		NAME = (CASE when $1 IS NOT NULL THEN $1 ELSE name END),
		AGE = (CASE when $2 IS NOT NULL THEN $2 ELSE age END)	
	  WHERE id = $3 RETURNING id, name, age",
        )
        .bind(name)
        .bind(age)
	.bind(id)
        .fetch_one(db)
        .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

	SimpleBroker::publish(DogChanged {
		mutation_type: MutationType::Created,
		id: res.id,
	});

	Ok(res.id)	
    }
    
	async fn delete_dog(&self, ctx: &Context<'_>, 
	#[graphql(desc = "(REQUIRED) The ID of the record to delete")] id: i32
		) -> Result<i32, String> {
        let db = match ctx.data::<PgPool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        let res = match sqlx::query_as::<_, Dog>(
		"DELETE FROM dogs WHERE id = $1"
        )
	.bind(id)
        .fetch_one(db)
        .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
	

	SimpleBroker::publish(DogChanged {
		mutation_type: MutationType::Created,
		id: res.id,
	});

	Ok(res.id)	
    }
}
