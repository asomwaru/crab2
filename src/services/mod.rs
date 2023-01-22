mod cargo;
mod ddg;
mod github;
mod yt;

use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchParams {
    pub query: String,
}

pub fn routes() -> Router {
    Router::new().route("/search", get(search_handler))
}

async fn search_handler(params: Query<SearchParams>) -> String {
    println!("{}", params.query);

    params.query.clone()
}
