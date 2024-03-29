#![allow(dead_code)]

mod http;
mod server;
mod website_handler;

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    let default_path = format!("{}/src/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path: {}", public_path);
    server.run(WebsiteHandler::new(public_path));
}
