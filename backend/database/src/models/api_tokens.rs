pub struct ApiToken {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub application_id: i64,
    pub user_id: i64,
    pub token: String,
    pub note: String,
    pub expires_at: Option<chrono::NaiveDateTime>,
}

pub struct CreateApiToken {
    pub application_id: i64,
    pub user_id: i64,
    pub token: String,
    pub note: String,
    pub expires_at: Option<chrono::NaiveDateTime>,
}
