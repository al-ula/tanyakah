use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use serde_json::json;
use std::collections::HashMap;
use serde::Deserialize;
use tracing::info;
mod data;
mod database;
mod render;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting");
    let router = Router::new()
        .get(routes::index)
        .push(Router::with_path("daftar").get(routes::register).post(routes::htmx::register_post))
        .push(Router::with_path("papan").get(routes::board))
        .push(Router::with_path("pesan").get(routes::message))
        .push(
            Router::with_path("assets")
                .push(Router::with_path("<**path>").get(StaticDir::new("assets"))),
        );
    let acceptor = TcpListener::new("127.0.0.1:8800").bind().await;
    Server::new(acceptor).serve(router).await;
}