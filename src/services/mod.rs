mod cargo;
mod ddg;
mod github;
mod google;
mod yt;

use google::Google;

use crate::helpers::SearchQuery;

use axum::extract::Query;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct SearchParams {
    pub query: String,
}

pub fn routes() -> Router {
    Router::new().route("/search", get(search_handler))
}

async fn search_handler(params: Query<SearchParams>) -> Redirect {
    println!("{}", params.query);

    // et [x, xs @ ..] = [1, 2, 3]

    let split = params.query.split_whitespace().collect::<Vec<_>>();
    let cmd = split[0];
    let query = split[1..].join(" ");

    match cmd {
        // "cargo" => cargo::search(params.query).await,
        // "ddg" => ddg::search(params.query).await,
        // "github" => github::search(params.query).await,
        // "yt" => yt::search(params.query).await,
        "g" => Google::search(Some(cmd.to_string()), query),
        _ => {
            info!("Invalid command... Redirecting to ddg");
            todo!("Redirect to DuckDuckGo");
        }
    }
}
