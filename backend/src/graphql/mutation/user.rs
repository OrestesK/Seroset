use async_graphql::{Context, InputObject, Object, Result};

use crate::{graphql::types::User, prisma::PrismaClient};

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub webrc: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let created = db
            .user()
            .create(input.username, input.webrc, vec![])
            .exec()
            .await?;

        Ok(created.into())
    }
}
