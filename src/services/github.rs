// GitHub users
// GitHub repos
// GitHub search

use crate::helpers::SearchQuery;

pub struct GitHub;

impl GitHub {
    pub fn search_user(&self, username: String) -> String {
        format!("https://github.com/{}", username)
    }

    pub fn search_repo(&self, repo: String) -> String {
        if !repo.contains('/') {
            return self.search_default(repo);
        }

        format!("https://github.com/{}", repo)
    }

    pub fn search_default(&self, query: String) -> String {
        format!("https://github.com/search?q={}", query)
    }
}

impl SearchQuery for GitHub {
    fn search(&self, _: Option<String>, query: String) -> String {
        let split = query.split_whitespace().collect::<Vec<_>>();
        let cmd = split[0].to_string();
        let search_query = {
            let temp = split[1..].join(" ").to_string();
            match temp.trim().is_empty() {
                true => query,
                false => temp,
            }
        };

        match cmd.as_ref() {
            "u" => self.search_user(search_query),
            "r" => self.search_repo(search_query),
            _ => self.search_default(search_query),
        }
    }
}
