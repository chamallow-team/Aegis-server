use rocket::config::Ident;
use rocket::log::LogLevel;
use rocket::{catchers, routes, Config};
use std::env;
use tracing::info;

mod catchers;
mod websocket;

struct Secret(String);

impl Secret {
    pub(self) fn get(&self) -> String {
        self.0.clone()
    }

    pub(self) fn init() -> Self {
        Self(env::var("SECRET_KEY").expect("SECRET_KEY must be set"))
    }
}

pub(crate) async fn server((port, address): (u16, String)) {
    let server = rocket::build()
        .register("/", catchers![catchers::not_found])
        .mount("/ws", routes![websocket::ws_listener])
        .manage(Secret::init())
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

    info!(target: "Api", "Starting the API on http://{address}:{port}");
    server.launch().await.expect("Failed to launch server");
}
