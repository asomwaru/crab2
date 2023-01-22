mod cargo;
mod ddg;
mod github;
mod yt;

use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct SearchParams {
    query: String,
}

pub fn routes() -> Router {
    Router::new().route("/search", get(cargo::handler))
}

async fn greet_handler(Query(params): Query<SearchParams>) -> &'static str {
    println("{}", params.query);

    "Hello, World!"
}
