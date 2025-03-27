use crate::database::users::SecretKey;
use hmac::{Hmac, Mac};
use rocket::config::Ident;
use rocket::log::LogLevel;
use rocket::{catchers, routes, Config};
use sqlx::SqlitePool;
use std::env;
use tracing::{error, info};

mod catchers;
pub mod responders;
mod routes;
mod websocket;

struct Secret(SecretKey);

impl Secret {
    pub(self) fn get(&self) -> Hmac<sha2::Sha256> {
        self.0.clone()
    }

    pub(self) fn init() -> Self {
        let s = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        Self(Hmac::new_from_slice(s.as_bytes()).expect("HMAC key is invalid"))
    }
}

type DbState = rocket::State<SqlitePool>;
type SecretKeyState = rocket::State<Secret>;

pub(crate) async fn server((port, address): (u16, String)) {
    let pool = match sqlx::SqlitePool::connect("aegis/server.sqlite").await {
        Ok(db) => db,
        Err(e) => {
            error!(target: "Api", "Cannot acquire the main database connection: {e:#?}");
            panic!("The database connection cannot be establish, it's a breaking error and should stop immediately the server.")
        }
    };

    let server = rocket::build()
        .register("/", catchers![catchers::not_found, catchers::bad_request])
        .mount("/ws", routes![websocket::ws_listener])
        .manage(Secret::init())
        .manage::<SqlitePool>(pool)
        .configure(rocket::Config {
            address: address.parse().expect("invalid IP address"),
            port,
            ident: Ident::try_new("Aegis_Server_1.0").expect("Wtf, why the ident is invalid?"),
            log_level: LogLevel::Critical,
            cli_colors: true,
            #[cfg(debug_assertions)]
            profile: Config::DEBUG_PROFILE,
            #[cfg(not(debug_assertions))]
            profile: Config::RELEASE_PROFILE,
            ..Default::default()
        });

    let server = routes::users::build(server);

    info!(target: "Api", "Starting the API on http://{address}:{port}");
    server.launch().await.expect("Failed to launch server");
}
