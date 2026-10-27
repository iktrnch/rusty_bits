use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, response::Html, routing::get};

use crate::error::AppError;
use crate::state::AppState;

async fn graphql_handler(
    State(state): State<Arc<AppState>>,
    request: GraphQLRequest,
) -> Result<GraphQLResponse, AppError> {
    let res = state
        .graphql_schema
        .execute(request.into_inner())
        .await
        .into();

    Ok(res)
}

async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub fn route() -> axum::Router<Arc<AppState>> {
    axum::Router::new().route("/graphql", get(graphiql).post(graphql_handler))
}
