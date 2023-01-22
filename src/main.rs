use axum::{routing::get, Router};
use clap::Parser;
use http::Method;
use tower_http::cors::{Any, CorsLayer};

use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;

mod helpers;
use helpers::port_in_range;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "127.0.0.1", value_parser = IpAddr::from_str,)]
    pub address: IpAddr,

    #[arg(short, long, default_value_t = 3000, value_parser = port_in_range,)]
    pub port: u16,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new().route("/", get(handler)).layer(cors);

    // run it
    let addr = SocketAddr::from((args.address, args.port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}
