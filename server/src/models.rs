use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Payload for creating or updating a user.
#[derive(Deserialize, ToSchema)]
pub struct UserPayload {
    /// Full name of the user
    pub name: String,
    /// Email address of the user
    pub email: String,
}

/// A user record returned from the database.
#[derive(Serialize, FromRow, ToSchema)]
pub struct User {
    /// Unique identifier
    pub id: Uuid,
    /// Full name of the user
    pub name: String,
    /// Email address of the user
    pub email: String,
}
