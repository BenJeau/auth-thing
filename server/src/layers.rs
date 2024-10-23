use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Request},
    Router,
};
use http::{header, HeaderName, Method};
use sentry::Hub;
use sentry_tower::{NewFromTopProvider, SentryHttpLayer, SentryLayer};
use std::{sync::Arc, time::Duration};
use tower::{
    layer::util::{Identity, Stack},
    ServiceBuilder,
};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    compression::{predicate::SizeAbove, CompressionLayer, CompressionLevel},
    cors::{AllowOrigin, Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::Span;

pub struct CommonTowerLayerBuilder {
    allowed_methods: Vec<Method>,
    allowed_origins: AllowOrigin,
    allowed_headers: Vec<HeaderName>,
    exposed_headers: Vec<HeaderName>,
    compression_level: CompressionLevel,
    compression_size: u16,
    timeout: u64,
    max_size: usize,
    tracing: bool,
    enable_sentry_layer: bool,
}

impl CommonTowerLayerBuilder {
    pub fn new() -> Self {
        Self {
            allowed_methods: vec![
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::DELETE,
                Method::PUT,
            ],
            allowed_origins: Any.into(),
            allowed_headers: vec![header::AUTHORIZATION, header::CONTENT_TYPE],
            exposed_headers: vec![
                "X-Version".parse().unwrap(),
                "X-Commit-SHA".parse().unwrap(),
            ],
            compression_level: CompressionLevel::Fastest,
            compression_size: 1024,
            timeout: 30,
            max_size: 1024 * 1024 * 250,
            tracing: true,
            enable_sentry_layer: true,
        }
    }

    pub fn build(
        self,
    ) -> CommonTowerLayer<impl Fn(&Request<Body>) -> Span + Send + Clone + 'static + Sync> {
        let cors_layer = CorsLayer::new()
            .allow_methods(self.allowed_methods)
            .allow_origin(self.allowed_origins)
            .expose_headers(self.exposed_headers)
            .allow_headers(self.allowed_headers);

        let compression_layer = CompressionLayer::new()
            .quality(self.compression_level)
            .compress_when(SizeAbove::new(self.compression_size));

        let timeout_layer = TimeoutLayer::new(Duration::from_secs(self.timeout));

        let size_limit_layer = DefaultBodyLimit::max(self.max_size);

        let tracing_layer = if self.tracing {
            Some(
                TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                    tracing::info_span!(
                        "request",
                        user_id = tracing::field::Empty,
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                }),
            )
        } else {
            None
        };

        let sentry_layer = if self.enable_sentry_layer {
            Some(
                ServiceBuilder::new()
                    .layer(sentry_tower::NewSentryLayer::<Request>::new_from_top())
                    .layer(sentry_tower::SentryHttpLayer::with_transaction()),
            )
        } else {
            None
        };

        CommonTowerLayer {
            cors_layer,
            compression_layer,
            timeout_layer,
            size_limit_layer,
            tracing_layer,
            sentry_layer,
        }
    }
}

type CTLCompressionLayer = CompressionLayer<SizeAbove>;
type CTLTracingLayer<F> = TraceLayer<SharedClassifier<ServerErrorsAsFailures>, F>;
type CTLSentryLayer = ServiceBuilder<
    Stack<
        SentryHttpLayer,
        Stack<SentryLayer<NewFromTopProvider, Arc<Hub>, Request<Body>>, Identity>,
    >,
>;

pub struct CommonTowerLayer<F: Fn(&Request<Body>) -> Span + Send + Clone + 'static + Sync> {
    cors_layer: CorsLayer,
    compression_layer: CTLCompressionLayer,
    timeout_layer: TimeoutLayer,
    size_limit_layer: DefaultBodyLimit,
    tracing_layer: Option<CTLTracingLayer<F>>,
    sentry_layer: Option<CTLSentryLayer>,
}

impl<F> CommonTowerLayer<F>
where
    F: Fn(&Request<Body>) -> Span + Send + Clone + 'static + Sync,
{
    pub fn apply_middlewares(self, router: Router) -> Router {
        let mut router = router
            .layer(self.size_limit_layer)
            .layer(self.timeout_layer)
            .layer(self.compression_layer)
            .layer(self.cors_layer);

        if let Some(layer) = self.tracing_layer {
            router = router.layer(layer);
        }

        if let Some(layer) = self.sentry_layer {
            router = router.layer(layer);
        }

        router
    }
}
