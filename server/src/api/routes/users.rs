//! This file will contain all routes used under the directory `/users/`

use crate::api::responders::Success;
use crate::api::{responders, DbState, SecretKeyState};
use crate::database::users;
use crate::database::users::hash;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::json;
use rocket::serde::{json::Json, Deserialize};
use rocket::{get, post, routes, Build, Request, Rocket};
use tracing::error;

/// Takes the api configuration and add all specific states and routes associated
pub(in crate::api) fn build(app: Rocket<Build>) -> Rocket<Build> {
    app.mount("/users", routes![login_route, signup_route, get_user_info])
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Login {
    username: String,
    passwd: String,
}

#[post("/login", data = "<login>")]
async fn login_route(
    db: &DbState,
    secret_key: &SecretKeyState,
    login: Json<Login>,
) -> Result<Success, responders::Error> {
    let mut db_conn = match db.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Cannot acquire the connection to the db: {e:#?}");
            return Err(responders::Error::InternalError(
                r#"{ "message:": "Cannot acquire the database connection" }"#.to_string(),
            ));
        }
    };

    let user =
        match users::get_user_by_username_and_passwd(&mut db_conn, &login.username, &login.passwd)
            .await
        {
            Ok(u) => match u {
                Some(u) => u,
                None => return Err(responders::Error::NotFound(
                    r#"{ "message": "This user doesn't exist or the credentials are invalid" }"#
                        .to_string(),
                )),
            },
            Err(e) => {
                error!(target: "LoginRoute", "An error occurred while acquiring the user: {e:#?}");
                return Err(responders::Error::InternalError(
                    r#"{ "message:" "An internal error occured" }"#.to_string(),
                ));
            }
        };

    match users::generate_jwt(&secret_key.get(), &user) {
        Ok(jwt) => Ok(Success(format!("{{ \"token\": {jwt:?} }}"))),
        Err(e) => {
            error!(target: "JwtGenerator", "Cannot generate the jwt: {e:#?}");
            Err(responders::Error::InternalError(
                r#"{ "message": "An internal error occured" }"#.to_string(),
            ))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Signup {
    username: String,
    nickname: String,
    passwd: String,
}

#[post("/signup", data = "<signup>")]
async fn signup_route(
    db: &DbState,
    secret_key: &SecretKeyState,
    signup: Json<Signup>,
) -> Result<Success, responders::Error> {
    let mut db_conn = match db.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Cannot acquire the connection to the db: {e:#?}");
            return Err(responders::Error::InternalError(
                r#"{ "message:": "Cannot acquire the database connection" }"#.to_string(),
            ));
        }
    };

    let action = users::create_user(
        &mut db_conn,
        &secret_key.get(),
        &users::User {
            username: signup.username.clone(),
            nickname: signup.nickname.clone(),
            passwd: signup.passwd.clone(),
        },
    )
    .await;

    match action {
        Ok(_) => Ok(Success("{ \"message\": \"account created\" }".to_string())),
        Err(e) => Err(e),
    }
}

/// A guard (middleware) for checking if the user is logged in
struct Token(String);

impl Token {
    fn get_token(&self) -> &str {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = String;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");

        match token {
            Some(token) => {
                // check validity
                Outcome::Success(Token(token.to_string()))
            }
            // token does not exist
            None => Outcome::Error((
                Status::Unauthorized,
                "You need to be authenticated".to_string(),
            )),
        }
    }
}

#[get("/@me")]
async fn get_user_info(db: &DbState, token: Token) -> Result<Success, responders::Error> {
    dbg!(token.get_token());
    let hashed_token = hash(token.get_token());
    let mut db_conn = match db.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Cannot acquire the connection to the db: {e:#?}");
            return Err(responders::Error::InternalError(
                r#"{ "message:": "Cannot acquire the database connection" }"#.to_string(),
            ));
        }
    };

    let u_opt = match users::get_user_by_hashed_jwt(&mut db_conn, &hashed_token).await {
        Ok(u) => u,
        Err(e) => {
            error!("Cannot query the user: {e:#?}");
            return Err(responders::Error::InternalError(
                r#"{ "message": "An internal error occured" }"#.to_string(),
            ));
        }
    };

    match u_opt {
        Some(u) => Ok(Success(
            json!({
                "username": u.username,
                "nickname": u.nickname,
            })
            .to_string(),
        )),
        None => Err(responders::Error::InternalError(
            r#"{ "message": "User not found" }"#.to_string(),
        )),
    }
}
