mod error;
mod graphql;
mod state;

use axum::Router;
use std::sync::Arc;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    // State
    let state = AppState::new().unwrap();
    // build our application with a single route
    let app = Router::new()
        .merge(graphql::route())
        .with_state(Arc::new(state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
