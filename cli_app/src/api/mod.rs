pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod request;
pub mod response;
pub mod v1;

use axum::Router;
use std::sync::Arc;

use crate::state::ApplicationState;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().nest("/v1", v1::configure(state))
}
