use async_graphql::*;
use futures_util::StreamExt;
use tokio_stream::Stream;
use crate::broker::SimpleBroker;

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
pub enum MutationType {
    Created,
    Updated,
    Deleted,
}

#[derive(Clone)]
pub struct DogChanged {
    pub mutation_type: MutationType,
    pub id: i32,
}

#[Object]
impl DogChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> i32 {
        self.id
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn dogs_changed(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = DogChanged> {
        SimpleBroker::<DogChanged>::subscribe().filter(move |evt| {
            let res = if let Some(mutation_type) = mutation_type {
                evt.mutation_type == mutation_type
            } else {
                true
            };

            async move { res }
        })
    }
}
