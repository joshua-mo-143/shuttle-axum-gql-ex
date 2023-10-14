use async_graphql::{context::Context, Object};
use sqlx::PgPool;

pub struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "partner"
    }

    async fn dogs(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Filter by specific ID")] id: Option<i32>,
        #[graphql(desc = "Filter by specific name")] name: Option<String>,
        #[graphql(desc = "Filter by exact age")] age: Option<i32>,
    ) -> Result<Option<Vec<Dog>>, String> {
        let db = match ctx.data::<PgPool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        let res = match sqlx::query_as::<_, Dog>(
            "SELECT * FROM dogs
		 WHERE (CASE when $1 is not null then (id = $1) else (id = id) end)
		AND (CASE WHEN $2 is not null then (name = $2) else (name = name) end)
		AND (CASE when $3 is not null then (age = $3) else (age = age) end)
		 ",
        )
        .bind(id)
        .bind(name)
        .bind(age)
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
