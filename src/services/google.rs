// Google search
// Gmail

use tracing::error;
use tracing::info;

use crate::helpers::SearchQuery;

pub struct Google;

impl SearchQuery for Google {
    fn search(cmd: Option<String>, query: String) -> String {
        if cmd.is_none() {
            error!("No argument specified for Google Items");
            return "/".to_string();
        }

        let command = cmd.unwrap();

        match command.as_str() {
            "g" => {
                info!("Found google search");
                format!("https://www.google.com/search?q={}", &query)
            }
            "gm" => {
                info!("Found gmail redirect");
                "https://mail.google.com/".to_string()
            }
            _ => {
                error!("No argument specified for Google Items");
                "/".to_string()
            }
        }
    }
}
