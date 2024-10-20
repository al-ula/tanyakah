use crate::db::DB;
use eyre::{eyre, Report, Result, WrapErr};
use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;
use crate::config::{Config, CONFIG};

mod data;
mod db;
mod render;
mod routes;
mod config;
mod auth;

#[tokio::main]
async fn main() -> Result<(), Report> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_env("TANYAKAH_LOG").unwrap_or(EnvFilter::new("info")))
        .init();
    info!("Starting");
    
    match Config::init() {
        Ok(_) => {info!("Config loaded");}
        Err(e) => {
            return Err(e);
        }
    };

    let config = match CONFIG.get() {
        None => {
            error!("Config not loaded");
            std::process::exit(1);
        }
        Some(c) => c.clone(),
    };

    // reset db on debug
    #[cfg(debug_assertions)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut stream = signal(SignalKind::interrupt())?;
        let db = config.db.clone();
        tokio::spawn(async move {
            if stream.recv().await.is_some() {
                info!("Received SIGINT, terminating...");
                let db = db;
                if std::fs::metadata(&db).is_err() {
                    warn!("Database file not found.");
                    std::process::exit(1);
                }
                std::fs::remove_file(&db).expect("Failed to remove database");
                info!("Database file removed.");
                std::process::exit(0);
            }
        });
    }

    // initialize db
    db::Db::init(CONFIG.get().unwrap().db.clone()).wrap_err("Failed to initialize database")?;
    match DB.get() {
        Some(_s) => info!("Database initialized"),
        None => return Err(eyre!("DB is not initialized")),
    };

    auth::init();
    
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
    let url = config.host.clone() + ":" + &config.port.to_string();
    let acceptor = TcpListener::new(url).bind().await;
    Server::new(acceptor).serve(router).await;
    Ok(())
}

