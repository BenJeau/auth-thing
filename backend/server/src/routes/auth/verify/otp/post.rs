use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use database::{logic, models::users::User};
use serde::{Deserialize, Serialize};
use totp::{CryptoAlgorithm, Totp};
use utoipa::ToSchema;

use crate::{error::Error, ServerState};

#[derive(Debug, Deserialize)]
pub struct VerifyOtpQuery {
    otp: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyOtpResponse {
    success: bool,
    redirect: Option<String>,
}

/// Verify a one-time password (OTP) for two-factor authentication
#[utoipa::path(
    post,
    path = "",
    tag = "Auth",
    params(
        ("slug", Path, description = "Application slug"),
        ("otp" = String, Query, description = "One-time password to verify")
    ),
    responses(
        (status = 200, description = "OTP verification successful", body = VerifyOtpResponse),
        (status = 400, description = "Invalid OTP provided"),
        (status = 403, description = "2FA is not enabled for this user"),
        (status = 404, description = "TOTP secret not found for user")
    )
)]
pub async fn verify_otp(
    State(state): State<ServerState>,
    Extension(user): Extension<User>,
    Query(query): Query<VerifyOtpQuery>,
) -> impl IntoResponse {
    if !user.two_factor_enabled {
        return Err(Error::TotpDisabled);
    }

    let secret = logic::users::get_two_factor_secret(&state.pool, user.id)
        .await?
        .ok_or_else(|| Error::TotpSecretNotFound)?;

    let totp = Totp::new(&secret, CryptoAlgorithm::Sha1, 6, 30, 0);

    let is_valid = totp.validate_otp(&query.otp, std::time::UNIX_EPOCH, &(1, 1).into())?;

    if !is_valid {
        return Err(Error::TotpInvalid);
    }

    Ok(Json(VerifyOtpResponse {
        success: true,
        redirect: None,
    }))
}
