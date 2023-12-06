use async_graphql::{EmptySubscription, Schema};

use crate::{
    graphql::{mutation::Mutation, query::Query},
    prisma::PrismaClient,
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the PrismaClient to the context
pub async fn build_schema() -> AppSchema {
    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Failed to create Prisma client");

    // push if you need
    // migrate if you need

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(client)
        .finish()
}
