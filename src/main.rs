use clap::Parser;
use dirs::home_dir;

use std::fs::create_dir_all;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;

mod helpers;
use helpers::port_in_range;

mod services;
use services::routes;

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
    let log_dir = format!(
        "{}/.config/crab2/logs",
        home_dir()
            .expect("Could not get home dir")
            .to_str()
            .expect("Could not get home dir as str")
    );
    create_dir_all(&log_dir).expect("Failed to create logging directory");

    tracing_subscriber::fmt()
        .with_writer(tracing_appender::rolling::hourly(log_dir, "crab2.log"))
        .init();

    let args = Cli::parse();

    let app = routes();

    // run it
    let addr = SocketAddr::from((args.address, args.port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
