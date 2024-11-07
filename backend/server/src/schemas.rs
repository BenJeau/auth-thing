use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Data needed to signup/create a new user
#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct SignupUserRequest {
    /// The name of the user to authenticate
    pub name: Option<String>,
    /// The username of the user to authenticate
    pub username: Option<String>,
    /// The email of the user to authenticate
    pub email: String,
    /// The password of the user to authenticate
    pub password: String,
}

/// Data needed to login a user
#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct LoginUserRequest {
    /// The email of the user to authenticate
    pub email: String,
    /// The password of the user to authenticate
    pub password: String,
}

/// Response from login request
#[derive(Serialize, ToSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserResponse {
    /// The JWT token created from login request that can be used to authenticate yourself
    pub jwt_token: String,
}
