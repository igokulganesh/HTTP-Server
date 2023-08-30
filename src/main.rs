#![allow(dead_code)]

mod server;
mod http;

use std::env;
use server::Server;
use http::WebsiteHandler;

fn main() {

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path.to_string());
    println!("Public Path: {}", public_path);
    let server: Server = Server::new("127.0.0.1:5070".to_string());
    
    server.run(WebsiteHandler::new(public_path))
}