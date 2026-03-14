use anyhow::Context;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

#[derive(Serialize)]
struct Responce {
    message: &'static str,
}

async fn hello_json() -> Result<(StatusCode, Json<Responce>), AppError> {
    let responce = Responce {
        message: generate_message().context("Failed to generate message")?,
    };

    Ok((StatusCode::OK, Json(responce)))
}

/// Generates the hello world message
fn generate_message() -> anyhow::Result<&'static str> {
    if rand::random() {
        anyhow::bail!("no message for you");
    }
    Ok("Hello, World!")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(hello_json))
        .layer(tower_http::catch_panic::CatchPanicLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listen")?;
    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}
