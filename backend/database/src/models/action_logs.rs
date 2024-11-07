use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

/// Action request log
#[derive(FromRow, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActionLog {
    /// Database ID of the log
    pub id: i64,
    /// Time when the log was created
    pub created_at: NaiveDateTime,
    /// Time when the log was modified
    pub updated_at: NaiveDateTime,
    /// Database ID of the user that made the request
    pub user_id: i64,
    /// IP address of the user that made the request
    pub ip_address: String,
    /// Browser user agent of the user that made the request
    pub user_agent: String,
    /// URI path of the request
    pub uri: String,
    /// HTTP method of the request
    pub method: String,
    // Opentelemetry trace ID of the request, can be used to correlate logs with a tool like Jaeger
    // pub trace_id: String,
}

/// Internal action request log
#[derive(Debug, Clone)]
pub struct CreateActionLog {
    pub user_id: i64,
    pub ip_address: String,
    pub user_agent: String,
    pub uri: String,
    pub method: String,
}
