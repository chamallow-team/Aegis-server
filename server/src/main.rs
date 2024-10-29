mod api;
mod database;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use sqlx::{ConnectOptions, Connection};
use std::env;
use std::path::Path;
use tracing::{error, info, trace, warn};

#[derive(Parser, Debug)]
struct CliConfig {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(name = "launch", about = "launch the server")]
    LaunchConfig {
        #[clap(long, short)]
        port: u16,
        #[clap(long, short)]
        address: String,
    },
    #[clap(name = "init", about = "Initialize the database")]
    Init,
    #[clap(name = "version", about = "The version of the server")]
    Version,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = CliConfig::parse();

    match config.command {
        Command::LaunchConfig { port, address } => launch_server(port, address).await,
        Command::Init => init_server().await,
        Command::Version => match env::var("AEGIS_VERSION").ok() {
            Some(v) => println!("{v}"),
            None => println!("An error occurred while retrieving the version"),
        },
    }
}

const GLOBAL_DATABASE_SCRIPT: &str = include_str!("../../data/databases_scripts/server.sql");

/// Initialize all required files and the database
async fn init_server() {
    println!("Please do not close the process if it don't move. The program is still doing it's job, just that he isn't very verbose.");
    println!("The program will automatically exit at the end.");
    println!();

    tracing_subscriber::fmt().init();

    if !Path::new("aegis").is_dir() {
        info!("Creating directory 'aegis'");
        if let Err(e) = std::fs::create_dir("aegis") {
            error!("Cannot create the directory 'aegis': {e:#?}");
            std::process::exit(1);
        }
    } else {
        warn!("Directory 'data' already exist")
    }

    // initialize database
    info!("Creating the main database");
    {
        let sqlite_file = sqlx::sqlite::SqliteConnectOptions::new()
            .filename("aegis/server.sqlite")
            .create_if_missing(true)
            .connect()
            .await;

        if let Err(e) = &sqlite_file {
            error!(target: "MainDb", "Cannot initialize the database at 'data/server.sqlite': {e:#?}");
            std::process::exit(1);
        }
        let mut sqlite = sqlite_file.unwrap();

        trace!(target: "MainDb", "Configuring the database...");
        // Execute the SQL script
        if let Err(e) = sqlx::raw_sql(GLOBAL_DATABASE_SCRIPT)
            .execute(&mut sqlite)
            .await
        {
            error!(target: "MainDb", "An error occurred while configuring the database: {e:#?}");
            std::process::exit(1);
        };
        info!(target: "MainDb", "Database configured");

        // Close the connection at the end
        if let Err(e) = sqlite.close().await {
            error!(target: "MainDb", "Cannot close the connection from 'data/server.sqlite': {e:#?}");
            std::process::exit(1);
        }
    }

    info!("Initialization finished.");
    std::process::exit(0);
}

/// Launch the server with the port and the address specified
async fn launch_server(port: u16, address: String) {
    // enable the tracer-subscriber
    tracing_subscriber::fmt().init();

    // check if the 'secret' is configured
    if std::env::var("SECRET_KEY").ok().is_none() {
        error!("You need to add a `SECRET_KEY` in the environment variables. Please refer to the documentation for further explanations.");
        std::process::exit(1);
    }

    let server_handle = tokio::spawn(async move { api::server((port, address)).await });

    //
    //
    //  SERVER END
    //
    //

    server_handle
        .await
        .expect("Server Listener failed to start");
}
