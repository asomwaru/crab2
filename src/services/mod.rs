mod cargo;
mod ddg;
mod github;
mod google;
mod yt;

use cargo::Cargo;
use ddg::DuckDuckGo;
use google::Google;

use crate::helpers::SearchQuery;

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use axum_extra::routing::SpaRouter;
use http::Method;
use serde::Deserialize;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Deserialize, Debug)]
struct SearchParams {
    pub query: String,
}

pub fn routes() -> Router {
    // Need to implement 404 errors
    Router::new()
        .merge(SpaRouter::new("/", "frontend").index_file("index.html"))
        .route("/search", get(search_handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST]),
        )
}

async fn search_handler(params: Query<SearchParams>) -> impl IntoResponse {
    let split = params.query.split_whitespace().collect::<Vec<_>>();
    let cmd = split[0].to_string();
    let query = split[1..].join(" ");

    let uri = match cmd.as_str() {
        "cargo" | "c" | "cr" => Cargo::search(None, query),
        "ddg" => DuckDuckGo::search(None, query),
        // "github" => github::search(params.query).await,
        // "yt" => yt::search(params.query).await,
        "g" | "gm" => Google::search(Some(cmd.to_string()), query),
        _ => {
            info!("Invalid command... Redirecting to ddg");

            if query.len() > 0 {
                DuckDuckGo::search(None, query)
            } else {
                "/".to_string()
            }
        }
    };

    Json(uri)
}
