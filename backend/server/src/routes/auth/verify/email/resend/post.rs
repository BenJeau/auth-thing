use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    Json,
};
use chrono::{Duration, Utc};
use database::{logic, models::users::User};
use password::hash_password;
use serde::Serialize;
use tokio::join;
use utoipa::ToSchema;

use crate::{emails::send_verification_email, Error, ServerState};

#[derive(Debug, Serialize, ToSchema)]
pub struct SuccessResponse {
    success: bool,
    message: Option<String>,
}

/// Resend email verification code
#[utoipa::path(
    post,
    path = "",
    tag = "Auth",
    params(
        ("slug" = String, Path, description = "Application slug")
    ),
    responses(
        (status = 200, body = SuccessResponse),
        (status = 400, description = "User already verified"),
        (status = 429, description = "Too many requests - must wait before requesting new code"),
        (status = 500, description = "Email service not configured")
    )
)]
pub async fn resend_verification_code(
    State(state): State<ServerState>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    if user.email_verified {
        return Err(Error::AlreadyVerified);
    }

    let Some(mailer) = state.mailer else {
        return Err(Error::MailerNotConfigured);
    };

    match user.verification_code_created_at {
        Some(created_at) => {
            let min_seconds = state.config.auth.email.resend_min_seconds;
            let now = Utc::now().naive_utc();

            if created_at + Duration::seconds(min_seconds as i64) < now {
                return Err(Error::TooManyRequests);
            }
        }
        None => {
            return Err(Error::NotVerified);
        }
    }

    let code = state
        .crypto
        .generate_random_numeric_string(state.config.auth.email.code_length);
    let hashed_code = hash_password(&code)?;

    let (update_result, send_result) = join!(
        logic::users::update_user_verification_code(
            &state.pool,
            user.id,
            &hashed_code,
            Utc::now().naive_utc(),
        ),
        send_verification_email(&code, &user.email, &mailer),
    );

    update_result?;
    send_result?;

    Ok(Json(SuccessResponse {
        success: true,
        message: Some("New verification code sent".to_string()),
    }))
}
