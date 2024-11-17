use axum::{
    extract::{Extension, Query, State},
    response::IntoResponse,
    Json,
};
use database::{logic, models::users::User};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{error::Result, ServerState};

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyEmailResponse {
    success: bool,
}

/// Verify email
#[utoipa::path(
    post,
    path = "",
    tag = "Auth",
    responses(
        (status = 200, body = VerifyEmailResponse)
    ),
    params(
        ("slug", Path, description = "Application slug"),
        ("token", Query, description = "Verification token")
    )
)]
pub async fn verify_email(
    State(state): State<ServerState>,
    Extension(user): Extension<User>,
    Query(payload): Query<VerifyEmailRequest>,
) -> Result<impl IntoResponse> {
    if user.email_verified {
        return Ok(Json(VerifyEmailResponse { success: true }));
    }

    let success = logic::users::verify_email(&state.pool, user.id, &payload.token).await?;

    Ok(Json(VerifyEmailResponse { success }))
}
