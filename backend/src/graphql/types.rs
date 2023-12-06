use async_graphql::{ComplexObject, SimpleObject};

use crate::prisma::user;

// Note: ideally, I would not need this file at all. I would just use the generated
// struct definitions from the prisma client, however with async-graphql I need
// to derive some traits to make it work.

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub webrc: String,
}

#[ComplexObject]
impl User {
    // pub async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
    //     let db = ctx.data::<PrismaClient>().unwrap();
    //
    //     Ok(db
    //         .post()
    //         .find_many(vec![post::user_id::equals(self.id.clone())])
    //         .exec()
    //         .await?
    //         .into_iter()
    //         .map(|p| p.into())
    //         .collect())
    // }
}

impl From<user::Data> for User {
    fn from(data: user::Data) -> User {
        User {
            id: data.id,
            username: data.username,
            webrc: data.webrc,
        }
    }
}

impl From<&user::Data> for User {
    fn from(data: &user::Data) -> User {
        User {
            id: data.id.clone(),
            username: data.username.clone(),
            webrc: data.webrc.clone(),
        }
    }
}
