use axum::{extract::{Path, State}, http::StatusCode, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{User, UserPayload};

/// List all users
#[utoipa::path(
    get,
    path = "/api/users",
    tag = "users",
    responses(
        (status = 200, description = "Successfully retrieved list of users", body = Vec<User>),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Create a new user
#[utoipa::path(
    post,
    path = "/api/users",
    tag = "users",
    request_body = UserPayload,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UserPayload>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    sqlx::query_as::<_, User>("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
        .bind(payload.name)
        .bind(payload.email)
        .fetch_one(&pool).await
        .map(|u| (StatusCode::CREATED, Json(u)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Get a single user by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "users",
    params(("id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found"),
    )
)]
pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool).await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

/// Update an existing user
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    tag = "users",
    params(("id" = Uuid, Path, description = "User ID")),
    request_body = UserPayload,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UserPayload>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>("UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *")
        .bind(payload.name)
        .bind(payload.email)
        .bind(id)
        .fetch_one(&pool).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Delete a user by ID
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "users",
    params(("id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
