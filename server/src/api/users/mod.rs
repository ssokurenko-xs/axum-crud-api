mod handlers;

use axum::{routing::{get, post}, Router};
use sqlx::PgPool;

pub use handlers::*;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/api/users", post(create_user).get(list_users))
        .route("/api/users/{id}", get(get_user).put(update_user).delete(delete_user))
}
