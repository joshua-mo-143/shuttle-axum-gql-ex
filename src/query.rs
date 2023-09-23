use sqlx::PgPool;
use async_graphql::{context::Context, Object};

pub struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "partner"
    }

    async fn dogs(&self, ctx: &Context<'_>) -> Result<Option<Vec<Dog>>, String> {
        let db = match ctx.data::<PgPool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        let res = match sqlx::query_as::<_, Dog>("SELECT * FROM dogs")
            .fetch_all(db)
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        Ok(Some(res))
    }
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Dog {
    pub id: i32,
    name: String,
    age: i32,
}

#[Object]
impl Dog {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn name(&self) -> String {
        self.name.clone()
    }
    async fn age(&self) -> i32 {
        self.age
    }
}
