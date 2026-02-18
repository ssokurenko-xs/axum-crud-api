use axum::{http::StatusCode, Json};
use uuid::Uuid;

use crate::models::{RegisterDevicePayload, UploadKeyResponse};

/// Register a new device
#[utoipa::path(
    post,
    path = "/api/devices/register",
    tag = "devices",
    request_body = RegisterDevicePayload,
    responses(
        (status = 200, description = "Device registered successfully"),
    )
)]
pub async fn register_device(
    Json(_payload): Json<RegisterDevicePayload>,
) -> StatusCode {
    StatusCode::OK
}

/// Request a pre-signed URL for file upload
#[utoipa::path(
    get,
    path = "/api/devices/uploadKey",
    tag = "devices",
    responses(
        (status = 200, description = "Pre-signed upload URL", body = UploadKeyResponse),
    )
)]
pub async fn upload_key() -> Json<UploadKeyResponse> {
    Json(UploadKeyResponse {
        url: Uuid::new_v4().to_string(),
    })
}
