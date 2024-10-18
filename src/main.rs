use crate::db::DB;
use eyre::{eyre, Report, Result, WrapErr};
use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use std::path::PathBuf;
use tracing::{info, warn};

mod data;
mod db;
mod render;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Report> {
    tracing_subscriber::fmt::init();
    info!("Starting");
    // reset db on debug
    #[cfg(debug_assertions)]
    {
        for i in 0..4 {
            let small_uid = small_uid::SmallUid::new()?;
            let small_uid_str = small_uid.to_string();
            let char_len = small_uid_str.chars().count();
            info!("SmallUid: {} ({} characters)", small_uid_str, char_len);
        };
        
        use tokio::signal::unix::{signal, SignalKind};
        let mut stream = signal(SignalKind::interrupt())?;
        tokio::spawn(async move {
            if (stream.recv().await).is_some() {
                info!("Received SIGINT, terminating...");
                if std::fs::metadata("./db/data.db").is_err() {
                    warn!("Database file not found.");
                    std::process::exit(1);
                }
                std::fs::remove_file("./db/data.db").expect("Failed to remove database");
                info!("Database file removed.");
                std::process::exit(0);
            }
        });
    }
    
    // initialize db
    db::initialize_db(PathBuf::from("db/data.db")).wrap_err("Failed to initialize database")?;
    match DB.get() {
        Some(_s) => info!("Database initialized"),
        None => return Err(eyre!("DB is not initialized")),
    };

    let router = Router::new()
        .get(routes::index)
        .push(Router::with_path("profil").get(routes::profile))
        .push(Router::with_path("papanku").get(routes::my_board))
        .push(Router::with_path("papan").get(routes::board))
        .push(Router::with_path("pesan").get(routes::msg_page))
        .push(Router::with_path("msg").push(Router::with_path("<msg_id>")))
        .push(Router::with_path("rpl").push(Router::with_path("<rpl_id>")))
        .push(Router::with_path("<board_id>"))
        .push(
            Router::with_path("assets")
                .push(Router::with_path("<**path>").get(StaticDir::new("assets"))),
        );
    let acceptor = TcpListener::new("127.0.0.1:8800").bind().await;
    Server::new(acceptor).serve(router).await;
    Ok(())
}
