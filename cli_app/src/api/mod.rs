pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod request;
pub mod response;
pub mod v1;

use axum::{Router, extract::MatchedPath, http::Request};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::ApplicationState;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url(
            "/v1/api-docs/openapi.json",
            crate::api::v1::ApiDoc::openapi(),
        ))
        .nest("/v1", v1::configure(state))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.

                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                )
            }),
        )
}
