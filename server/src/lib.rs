mod api;
pub mod models;

use axum::{http::{HeaderValue, Method}, routing::get, Router};
use models::{RegisterDevicePayload, UploadKeyResponse, User, UserPayload};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "User Management API",
        version = "1.0.0",
        description = "CRUD API for managing users built with Axum + SQLx"
    ),
    paths(
        api::users::list_users,
        api::users::create_user,
        api::users::get_user,
        api::users::update_user,
        api::users::delete_user,
        api::devices::register_device,
        api::devices::upload_key,
    ),
    components(schemas(User, UserPayload, RegisterDevicePayload, UploadKeyResponse)),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "devices", description = "Device management endpoints"),
    )
)]
pub struct ApiDoc;

pub async fn run() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to connect to DB");
    sqlx::migrate!().run(&pool).await.expect("Migrations failed");

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api", get(|| async { "Welcome to the User Management API!" }))
        .merge(api::users::router())
        .merge(api::devices::router())
        .layer(cors)
        .with_state(pool);

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    println!("🚀 Server running on port {}", port);
    println!("📖 Swagger UI → http://localhost:{}/swagger-ui/", port);
    println!("📄 OpenAPI JSON → http://localhost:{}/api-docs/openapi.json", port);

    axum::serve(listener, app).await.unwrap();
}
