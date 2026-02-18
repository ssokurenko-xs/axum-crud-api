mod handlers;

use axum::{routing::{get, post}, Router};
use sqlx::PgPool;

pub use handlers::*;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/api/devices/register", post(register_device))
        .route("/api/devices/uploadKey", get(upload_key))
}
