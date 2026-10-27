use axum::http::StatusCode;
use axum::response::IntoResponse;

pub struct AppError(StatusCode, anyhow::Error);

impl From<(StatusCode, anyhow::Error)> for AppError {
    fn from((status_code, value): (StatusCode, anyhow::Error)) -> Self {
        tracing::error!(error = ?value, "Unhandled internal server error");

        Self(status_code, value)
    }
}

impl From<StatusCode> for AppError {
    fn from(value: StatusCode) -> Self {
        tracing::error!("Unhandled undocumented internal server error");

        Self(
            value,
            anyhow::anyhow!(value.canonical_reason().unwrap_or("")),
        )
    }
}

// This allows ? to automatically convert anyhow::Error to AppError
impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        tracing::error!(error = ?value, "Unhandled internal server error");

        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!(
                StatusCode::INTERNAL_SERVER_ERROR
                    .canonical_reason()
                    .unwrap_or("Internal server error")
            ),
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.0, format!("{:?}", self.1)).into_response()
    }
}
