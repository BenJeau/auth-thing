use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::ServerState;

#[tracing::instrument(skip_all)]
pub async fn version_middleware(
    State(state): State<ServerState>,
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        "X-Version",
        state.version.version.clone().try_into().unwrap(),
    );
    response.headers_mut().insert(
        "X-Commit-SHA",
        state.version.commit_sha.clone().try_into().unwrap(),
    );

    response
}
