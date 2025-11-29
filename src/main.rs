#![deny(unused_extern_crates)]

mod database;
mod error;
mod handlers;
mod models;
mod rendering;

use crate::database::setup_database;
use crate::handlers::{create_paste_handler, show_form, view_paste_handler};
use anyhow::Result;
use axum::{Router, routing::get};
use clap::Parser;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt};

/// A minimalist markdown pastebin.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Port to listen on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// Recreate the database on startup
    #[arg(long, default_value_t = false)]
    recreate_db: bool,

    /// Enable verbose logging (debug level)
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

const DB_FILENAME: &str = "pastes.db";

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };
    let filter = EnvFilter::builder()
        .with_default_directive(log_level.into())
        .from_env_lossy();

    fmt().with_env_filter(filter).init();

    // handle database recreation
    if cli.recreate_db {
        info!("`--recreate-db` flag set. Deleting old database file.");
        match std::fs::remove_file(DB_FILENAME) {
            Ok(_) => info!("Successfully deleted {}", DB_FILENAME),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                info!("Database file not found, nothing to delete.");
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    let pool = setup_database().await?;

    let app = Router::new()
        .route("/", get(show_form).post(create_paste_handler))
        .route("/p/:id", get(view_paste_handler))
        .with_state(pool);

    // run app
    let addr = SocketAddr::from(([127, 0, 0, 1], cli.port));
    let listener = TcpListener::bind(addr).await?;
    info!("🚀 Server listening on http://{}", addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
