// crates.io search

use axum::response::Redirect;

use crate::helpers::SearchQuery;

pub struct Cargo;

impl SearchQuery for Cargo {
    fn search(_: Option<String>, query: String) -> Redirect {
        let uri = format!("https://crates.io/search?q={}", query);
        Redirect::to(&uri)
    }
}
