use async_graphql::InputObject;

#[derive(InputObject)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
