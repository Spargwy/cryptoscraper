use axum::{
    routing::get,
    extract::Query,
    Json, Router,
    response::IntoResponse,
    http::StatusCode,
};
use serde::Deserialize;
use crate::scraper::fetch_prices;

#[derive(Debug, Deserialize)]
pub struct PricesQuery {
    ids: String, // например: bitcoin,ethereum
}

pub fn create_router() -> Router {
    Router::new().route("/prices", get(get_prices))
}

async fn get_prices(Query(query): Query<PricesQuery>) -> impl IntoResponse {
    let coins: Vec<&str> = query.ids.split(',').collect();
    match fetch_prices(&coins).await {
        Ok(data) => Json(data).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch prices").into_response(),
    }
}
