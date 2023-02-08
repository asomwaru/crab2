// DuckDuckGo search

use crate::helpers::SearchQuery;

pub struct DuckDuckGo;

impl SearchQuery for DuckDuckGo {
    fn search(&self, _: Option<String>, query: String) -> String {
        format!("https://duckduckgo.com/?q={}", query)
    }
}
