use axum::{ extract::{ Path, State }, http::StatusCode, routing::{ get, post }, Json, Router };
use serde::{ Deserialize, Serialize };
use sqlx::{ postgres::PgPoolOptions, FromRow, PgPool };
use std::env;
use utoipa::{ OpenApi, ToSchema };
use utoipa_swagger_ui::SwaggerUi;

// ── Schemas ──────────────────────────────────────────────────────────────────

/// Payload for creating or updating a user.
#[derive(Deserialize, ToSchema)]
struct UserPayload {
    /// Full name of the user
    name: String,
    /// Email address of the user
    email: String,
}

/// A user record returned from the database.
#[derive(Serialize, FromRow, ToSchema)]
struct User {
    /// Auto-incremented primary key
    id: i32,
    /// Full name of the user
    name: String,
    /// Email address of the user
    email: String,
}

// ── OpenAPI spec ──────────────────────────────────────────────────────────────

#[derive(OpenApi)]
#[openapi(
    info(
        title = "User Management API",
        version = "1.0.0",
        description = "A simple CRUD API for managing users built with Axum + SQLx"
    ),
    paths(list_users, create_user, get_user, update_user, delete_user),
    components(schemas(User, UserPayload)),
    tags((name = "users", description = "User management endpoints"))
)]
struct ApiDoc;

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to connect to DB");
    sqlx::migrate!().run(&pool).await.expect("Migrations failed");

    let app = Router::new()
        // Swagger UI at /swagger-ui  |  raw JSON at /api-docs/openapi.json
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api", get(root))
        .route("/api/users", post(create_user).get(list_users))
        .route("/api/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .with_state(pool);

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    println!("🚀 Server running on port {}", port);
    println!("📖 Swagger UI → http://localhost:{}/swagger-ui/", port);
    println!("📄 OpenAPI JSON → http://localhost:{}/api-docs/openapi.json", port);

    axum::serve(listener, app).await.unwrap();
}

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn root() -> &'static str {
    "Welcome to the User Management API!"
}

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
async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
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
async fn create_user(
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
    params(("id" = i32, Path, description = "User ID")),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found"),
    )
)]
async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
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
    params(("id" = i32, Path, description = "User ID")),
    request_body = UserPayload,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
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
    params(("id" = i32, Path, description = "User ID")),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx
        ::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}