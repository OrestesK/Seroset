use async_graphql::{Context, InputObject, Object, Result};

use crate::{graphql::types::User, prisma::user, prisma::PrismaClient};

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub webrc: String,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub id: i32,
    pub webrc: String,
}

#[derive(InputObject)]
pub struct DeleteUserInput {
    pub id: i32,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let db: &PrismaClient = ctx.data::<PrismaClient>().unwrap();

        let created: user::Data = db
            .user()
            .create(input.username, input.webrc, vec![])
            .exec()
            .await?;

        Ok(created.into())
    }

    pub async fn edit_user(&self, ctx: &Context<'_>, input: UpdateUserInput) -> Result<User> {
        let db: &PrismaClient = ctx.data::<PrismaClient>().unwrap();

        let updated: user::Data = db
            .user()
            .update(
                user::id::equals(input.id),
                vec![user::webrc::set(input.webrc)],
            )
            .exec()
            .await?;

        Ok(updated.into())
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, input: DeleteUserInput) -> Result<User> {
        let db: &PrismaClient = ctx.data::<PrismaClient>().unwrap();

        let deleted: user::Data = db.user().delete(user::id::equals(input.id)).exec().await?;

        Ok(deleted.into())
    }
}
