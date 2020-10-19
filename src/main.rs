#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod website_handler;
mod thread_pool;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let default_thread_count = 4;
    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run(WebsiteHandler::new(public_path, default_thread_count));
}