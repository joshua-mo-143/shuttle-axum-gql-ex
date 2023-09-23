use sqlx::PgPool;

mod broker;
mod mutation;
mod query;
mod router;
mod subscription;

use router::init_router;

#[shuttle_runtime::main]
async fn shuttle_main(#[shuttle_shared_db::Postgres] db: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&db).await.unwrap();

    let router = init_router(db);

    Ok(router.into())
}
