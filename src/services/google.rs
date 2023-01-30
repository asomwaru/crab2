// Google search
// Gmail

use axum::response::Redirect;
use tracing::error;

use crate::helpers::SearchQuery;

pub struct Google;

impl SearchQuery for Google {
    fn search(cmd: Option<String>, query: String) -> Redirect {
        if cmd.is_none() {
            error!("No argument specified for Google Items");
            return Redirect::permanent("/");
        }

        let command = cmd.unwrap();

        match command.as_str() {
            "g" => {
                let uri = format!("https://www.google.com/search?q={}", &query);
                Redirect::permanent(&uri)
            }
            "gm" => Redirect::permanent("https://mail.google.com/"),
            _ => {
                error!("No argument specified for Google Items");
                return Redirect::permanent("/");
            }
        }
    }
}
