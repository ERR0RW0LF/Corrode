use std::{fmt::Debug, time::Duration};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version};
use serde::Deserialize;
use thiserror::Error;
use axum_login::{AuthUser, AuthnBackend};
use sqlx::{FromRow, PgPool};
use tokio::time::sleep;
use uuid::Uuid;


#[derive(Error, Debug)]
pub enum AuthError {
    #[error("credentials given are incorrect")]
    CredentialsIncorrect,

    #[error("argon2 error")]
    Argon2Error,
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
    users: PgPool
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

use rand::{Rng, RngExt};

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = AuthError;

    async fn authenticate(
        &self,
        cred: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error>
    {
        let secret = std::env::var("PASSWORD_PEPPER").unwrap();
        let argon2 = Argon2::new_with_secret(
            secret.as_bytes(), 
            Algorithm::default(), 
            Version::default(), 
            Params::default()
        ).unwrap_or(return Err(Self::Error::Argon2Error));
        
        let mut rng = rand::rng();

        let base_delay_millis = 13;
        let base_delay_micros = 46;
        let jitter_millis = rng.random_range(-base_delay_millis..base_delay_millis);
        let jitter_micros = rng.random_range(-base_delay_micros..base_delay_micros);

        sleep(Duration::from_millis((base_delay_millis + jitter_millis) as u64) + Duration::from_micros((base_delay_micros + jitter_micros) as u64));



        let user: Self::User = if let Ok(Some(t)) = sqlx::query_as("select * from users where username = $1").bind(cred.username).fetch_optional(&self.users).await {
            t
        } else {
            return Ok(None);
        };

        match verify_password(&cred.password, &user.password_hash, argon2) {
            Ok(true) => {
                Ok(Some(user))
            },
            Ok(false) => {
                Ok(None)
            },
            Err(_) => {
                Err(Self::Error::Argon2Error)
            }
        }
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> impl Future<Output = Result<Option<Self::User>, Self::Error>> + Send
    {
        todo!("get_user based on user_id")
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
    Ok(res.is_ok() as bool)
}