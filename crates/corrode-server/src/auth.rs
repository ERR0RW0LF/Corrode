use std::collections::HashMap;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version, password_hash};
use thiserror::Error;
use axum_login::{AuthUser, AuthnBackend};
use sqlx::FromRow;
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
    users: HashMap<String, User>
}

#[derive(Clone)]
pub struct Credentials {
    user_name: String,
    user_password: String,
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


impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = AuthError;

    async fn authenticate(
        &self,
        Credentials { user_name, user_password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error>
    {
        if let Some(user) = self.users.get(&user_name) {
            let res = verify_password(&user_password, &user.password_hash);
            match res {
                Ok(true) => {
                    return Ok(Some(user.clone()));
                },
                _ => {
                    return Err(Self::Error::CredentialsIncorrect.into());
                }
            }
        } else {
            return Err(Self::Error::CredentialsIncorrect.into());
        }
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> impl Future<Output = Result<Option<Self::User>, Self::Error>> + Send
    {
        
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

fn verify_password(password: &str, pw_hash: &str) -> anyhow::Result<bool> {
    let secret = std::env::var("PASSWORD_PEPPER").unwrap();
    let argon2 = Argon2::new_with_secret(
        secret.as_bytes(), 
        Algorithm::default(), 
        Version::default(), 
        Params::default()
    )?;

    let res = argon2.verify_password(password.as_bytes(), pw_hash);
    Ok(res.is_ok() as bool)
}