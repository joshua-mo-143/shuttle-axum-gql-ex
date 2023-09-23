use crate::query::Dog;
use async_graphql::{context::Context, Object};
use sqlx::PgPool;
use crate::broker::SimpleBroker;
use crate::subscription::{DogChanged, MutationType};

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
}
