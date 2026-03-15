pub mod login;
pub mod posts;

use axum::Router;
use serde::{Deserialize, Serialize};

use crate::state::ApplicationState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
