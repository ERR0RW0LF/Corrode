use std::{fmt::Debug};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version};
use serde::Deserialize;
use thiserror::Error;
use axum_login::{AuthUser, AuthnBackend};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;


#[derive(Error, Debug)]
pub enum AuthError {
    #[error("argon2 error was parsed")]
    Argon2Error(String),

    #[error("sqlx error was parsed")]
    SqlxError(String),

    #[error("anyhow error was parsed")]
    AnyhowError(String),
}

impl From<sqlx::Error> for AuthError {
    fn from(value: sqlx::Error) -> Self {
        Self::SqlxError(value.to_string())
    }
}

impl From<argon2::Error> for AuthError {
    fn from(value: argon2::Error) -> Self {
        Self::Argon2Error(value.to_string())
    }
}

impl From<anyhow::Error> for AuthError {
    fn from(value: anyhow::Error) -> Self {
        Self::AnyhowError(value.to_string())
    }
}



#[derive(Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Clone)]
pub struct Backend {
    users: PgPool,
    pepper: String
}

impl Backend {
    pub fn new(users: PgPool, pepper: String) -> Self {
        Backend { users , pepper }
    }
}


#[derive(Clone, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("role", &self.role)
            .field("password_hash", &"[redacted]")
            .finish()
    }
}


impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}


impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = AuthError;

    async fn authenticate(
        &self,
        cred: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error>
    {
        let secret = self.pepper.clone();
        let argon2 = Argon2::new_with_secret(
            secret.as_bytes(), 
            Algorithm::default(), 
            Version::default(), 
            Params::default()
        )?;

        let user: Self::User = match sqlx::query_as("select * from users where username = $1").bind(cred.username).fetch_optional(&self.users).await? {
            Some(t) => t,
            None => {
                let _ = verify_password("a", "b", argon2);
                return Ok(None);
            }
        };

        if verify_password(&cred.password, &user.password_hash, argon2)? {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> impl Future<Output = Result<Option<Self::User>, Self::Error>> + Send
    {
        let pool = self.users.clone();
        async move {
            Ok(sqlx::query_as("select * from users where id = $1").bind(user_id).fetch_optional(&pool).await?)
        }
    }
}






fn hash_password(password: &str) -> anyhow::Result<String>{
    let secret = std::env::var("PASSWORD_PEPPER").unwrap();
    let argon2 = Argon2::new_with_secret(
        secret.as_bytes(), 
        Algorithm::default(), 
        Version::default(), 
        Params::default()
    )?;

    let password_hash = argon2.hash_password(password.as_bytes())?.to_string();
    Ok(password_hash)
}

fn verify_password(password: &str, pw_hash: &str, argon2: Argon2<'_>) -> anyhow::Result<bool> {
    let res = argon2.verify_password(password.as_bytes(), pw_hash);
    Ok(res.is_ok())
}