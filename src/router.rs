use std::sync::Arc;
use crate::{
    mutation::Mutation,
    query::{Dog, Query},
    subscription::Subscription,
};
use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_axum::*;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sqlx::PgPool;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio_stream::wrappers::BroadcastStream;

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub fn init_router(db: PgPool) -> Router {
    let schema = Schema::build(Query, Mutation, Subscription)
        .data(db)
        .finish();

    // start the http server
    Router::new()
        .route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema))
}
