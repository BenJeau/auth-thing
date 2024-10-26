use axum::{extract::State, response::Html, routing::get, Json, Router};
use utoipa::{openapi, OpenApi};
use utoipa_axum::router::OpenApiRouter;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Auth Thing REST API",
        description = "A [Axum](https://github.com/tokio-rs/axum) Rust based API REST endpoints for the Auth Thing system interfacing with SQLite via [sqlx](https://github.com/launchbadge/sqlx). This documentation is generated using [utoipa](https://github.com/juhaku/utoipa).

## How to use

Select a tag (category) to reveal information about the endpoints and select an endpoint to test them. To query the API, you'll need to authenticate yourself with one of the methods below or by clicking on any of the locks.

## Authentication

All endpoints are protected except for auth endpoints. You either need to authenticate with one of the following ways all within the `Authorization` request header:
1. provide a JWT as a bearer token - `Bearer JWT_TOKEN`
2. provide username + password as basic authentication - `Basic base64(username:password)`
3. provide a API token as token authentication - `Token API_TOKEN`

Only the documentation `/api/v1/docs` and the authentication `/api/v1/auth` endpoints are not protected.

## Clients

There's no official API client yet, but thanks to the OpenAPI documentation, you can generate your own HTTP client using something like [OpenAPI Generator](https://openapi-generator.tech/) with the JSON from the OpenAPI docs located at `/api/v1/docs/openapi.json`.",
        contact(
            name = "Benoît Jeaurond",
            email = "benoit@jeaurond.dev"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/license/MIT"
        )
    ),
    servers(
        (url = "/api/v1")
    ),
    tags(
        (name = "Health", description = "Overall health check for the service"),
        (name = "Users", description = "User management"),
    ),
)]
struct ApiDoc;

pub fn axum_openapi_router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
}

async fn openapi_json(State(openapi): State<openapi::OpenApi>) -> Json<serde_json::Value> {
    Json(serde_json::to_value(openapi).unwrap())
}

async fn scalar_api_html(State(openapi): State<openapi::OpenApi>) -> Html<String> {
    let mut scalar_html = include_str!("../../scalar/index.html").to_string();
    let scalar_js = include_str!("../../scalar/scalar.js");
    let openapi_spec = serde_json::to_string(&openapi).unwrap();

    scalar_html = scalar_html.replace("OPENAPI_SPEC", &openapi_spec);
    scalar_html = scalar_html.replace("SCALAR_JS", scalar_js);

    Html(scalar_html)
}

pub fn router(openapi: openapi::OpenApi) -> Router {
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/", get(scalar_api_html))
        .with_state(openapi)
}
