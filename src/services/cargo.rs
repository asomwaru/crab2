// crates.io search

use crate::helpers::SearchQuery;

pub struct Cargo;

impl SearchQuery for Cargo {
    fn search(&self, _: Option<String>, query: String) -> String {
        if query.len() > 0 {
            format!("https://crates.io/search?q={}", query)
        } else {
            "https://crates.io".to_string()
        }
    }
}
