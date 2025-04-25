use crate::config::{Config, CONFIG};
use crate::db::DB;
use eyre::{eyre, Result, Report, WrapErr};
use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use small_uid::SmallUid;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

mod auth;
mod config;
mod data;
mod db;
mod middleware;
mod render;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_env("TANYAKAH_LOG").unwrap_or(EnvFilter::new("info")))
        .init();
    info!("Starting");

    let some_uid = match SmallUid::try_from("GSqzvxLPHT8".to_string()) {
        Ok(u) => u,
        Err(e) => {
            info!("e: {:#?}", e);
            return Err(Report::from(e));
        }
    };
    info!("some_uid: {:?}", some_uid.to_string());

    match Config::init() {
        Ok(_) => {
            info!("Config loaded");
        }
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
        .hoop(middleware::verify_auth)
        .get(routes::index)
        .post(routes::htmx::register)
        .push(
            Router::with_path("msg")
                .hoop(middleware::verify_hx_request)
                .post(routes::htmx::send_message)
                .get(routes::htmx::get_messages),
        )
        .push(Router::with_path("msg/<msg_id>").get(routes::message_view))
        .push(Router::with_path("rpl").hoop(middleware::verify_hx_request).post(routes::htmx::send_reply))
        .push(Router::with_path("<board_id>").get(routes::board_view))
        .push(
            Router::with_path("assets")
                .push(Router::with_path("<**path>").get(StaticDir::new("assets"))),
        );
    let url = config.host.clone() + ":" + &config.port.to_string();
    let acceptor = TcpListener::new(url).bind().await;
    Server::new(acceptor).serve(router).await;
    Ok(())
}


