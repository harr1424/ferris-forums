use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub is_moderator: bool,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, argon2::password_hash::Error> {
        let parsed_stored_hash = PasswordHash::new(&self.password_hash)?;

        let result = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_stored_hash)
            .is_ok();

        Ok(result)
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub is_moderator: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DbAddUser {
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub is_moderator: bool,
    pub created_at: DateTime<Utc>,
}
