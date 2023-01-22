use axum::routing::get;

pub async fn handler() -> &'static str {
    "Hello, World!"
}
