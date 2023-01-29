// DuckDuckGo search

use axum::response::Redirect;

use crate::helpers::SearchQuery;

pub struct DuckDuckGo;

impl SearchQuery for DuckDuckGo {
    fn search(_: Option<String>, query: String) -> Redirect {
        let uri = format!("https://duckduckgo.com/?q={}", query);
        Redirect::to(&uri)
    }
}
