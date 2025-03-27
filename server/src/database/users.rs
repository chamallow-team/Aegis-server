use crate::api::responders;
use hmac::Hmac;
use jwt::SignWithKey;
use sha2::{Digest, Sha256};
use sqlx::{FromRow, SqliteConnection};
use std::collections::BTreeMap;
use tracing::error;

pub type SecretKey = Hmac<Sha256>;
pub type Jwt = String;

#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub nickname: String,
    pub username: String,
    pub passwd: String,
}

pub fn generate_jwt(secret_key: &SecretKey, user: &User) -> Result<Jwt, String> {
    let mut claims = BTreeMap::new();
    claims.insert("nickname", user.nickname.clone());
    claims.insert("password", user.passwd.clone());

    match claims.sign_with_key(secret_key) {
        Ok(jwt) => Ok(jwt),
        Err(e) => Err(e.to_string()),
    }
}

pub fn hash(s: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(s.as_bytes());

    // read hash digest and consume hasher
    let result = hasher.finalize();
    hex::encode(result)
}

/// Create a user and returns the jwt for further connections and access requirements
pub async fn create_user(
    db: &mut SqliteConnection,
    secret_key: &SecretKey,
    user: &User,
) -> Result<Jwt, responders::Error> {
    // first, check if the user already exists
    if get_user_by_username(db, &user.username)
        .await
        .ok()
        .flatten()
        .is_some()
    {
        return Err(responders::Error::Conflict(
            r#"{ "message": "The user already exists" }"#.to_string(),
        ));
    }

    // generate the jwt
    let jwt = match generate_jwt(secret_key, user) {
        Ok(jwt) => jwt,
        Err(e) => {
            error!(target: "JwtGenerator", "Cannot generate the jwt key: {e:#?}");
            return Err(responders::Error::InternalError(
                r#"{ "message": "An internal error occured" }"#.to_string(),
            ));
        }
    };

    let hashed_jwt = hash(&jwt);

    // store the user in the db
    let req = sqlx::query(
        "INSERT INTO users (username, nickname, passwd, hashed_jwt) VALUES (?, ?, ?, ?);",
    )
    .bind(user.username.clone())
    .bind(user.nickname.clone())
    .bind(user.passwd.clone())
    .bind(hashed_jwt.clone())
    .execute(db)
    .await;

    match req {
        Ok(_) => Ok(jwt),
        Err(e) => {
            error!(target: "CreateUserService", "Cannot insert the new user into the db: {e:#?}");
            Err(responders::Error::InternalError(
                r#"{ "message": "An internal error occurred" }"#.to_string(),
            ))
        }
    }
}

pub async fn get_user_by_hashed_jwt(
    db: &mut SqliteConnection,
    hashed_jwt: &String,
) -> Result<Option<User>, String> {
    let req = sqlx::query_as("SELECT username, nickname, passwd FROM users WHERE hashed_jwt=?;")
        .bind(hashed_jwt)
        .fetch_optional(db)
        .await;

    dbg!(&req);
    dbg!(&hashed_jwt);

    match req {
        Ok(u) => Ok(u),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_user_by_username(
    db: &mut SqliteConnection,
    username: &String,
) -> Result<Option<User>, String> {
    let req = sqlx::query_as("SELECT username, nickname, passwd FROM users WHERE username=?;")
        .bind(username)
        .fetch_optional(db)
        .await;

    match req {
        Ok(u) => Ok(u),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_user_by_username_and_passwd(
    db: &mut SqliteConnection,
    username: &str,
    passwd: &str,
) -> Result<Option<User>, String> {
    let req = sqlx::query_as(
        "SELECT username, nickname, passwd FROM users WHERE username=? AND passwd=?;",
    )
    .bind(username)
    .bind(passwd)
    .fetch_optional(db)
    .await;

    match req {
        Ok(u) => Ok(u),
        Err(e) => Err(e.to_string()),
    }
}
