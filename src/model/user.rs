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

#[cfg(test)]
mod user_model_tests {
    use super::*;
    #[test]
    fn test_hash_password_success() {
        let password = "strongpassword";
        let result = User::hash_password(password);
        assert!(
            result.is_ok(),
            "Password hashing failed: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_verify_password_success() {
        let password = "strongpassword";
        let password_hash = User::hash_password(password).unwrap();

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            password_hash,
            is_moderator: false,
            created_at: Utc::now(),
        };

        let result = user.verify_password(password);
        assert!(result.unwrap(), "Password verification failed");
    }

    #[test]
    fn test_verify_password_failure() {
        let password = "strongpassword";
        let wrong_password = "wrongpassword";
        let password_hash = User::hash_password(password).unwrap();

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            password_hash,
            is_moderator: false,
            created_at: Utc::now(),
        };

        let result = user.verify_password(wrong_password);
        assert!(!result.unwrap(), "Password verification should have failed");
    }
}
