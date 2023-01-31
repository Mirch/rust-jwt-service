use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::models::UserRole;

const JWT_SECRET: &[u8] = b"JWT_TOKEN_SECRET";

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
}

pub fn build_token(username: String, role: UserRole, token_duration: i64) -> Result<String> {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::seconds(token_duration))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username,
        role: role.into(),
        exp: expiration_time,
    };

    let header = Header::new(Algorithm::HS512);
    Ok(encode(
        &header,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )?)
}

pub fn validate_token(token: String) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS512);
    validation.leeway = 0;
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET), &validation)?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use claim::assert_err;

    use crate::models::UserRole;

    use super::{build_token, validate_token};

    #[test]
    pub fn build_token_returns_valid_jwt() -> anyhow::Result<()> {
        let username = "test-username";
        let role = UserRole::Admin;

        // testing build_token
        let token = build_token(username.to_string(), role, 60)?;

        // testing validate_token
        let claims = validate_token(token)?;

        assert_eq!(username, claims.sub);
        assert_eq!(role, claims.role.into());
        Ok(())
    }

    #[test]
    pub fn validating_an_invalid_token_throws_error() -> anyhow::Result<()> {
        let username = "test-username";
        let role = UserRole::Admin;
        let token_duration = 1; // second

        let token = build_token(username.to_string(), role, token_duration)?;
        thread::sleep(Duration::from_secs(2)); // sleeping for 2 seconds to make sure the token has expired

        let claims = validate_token(token);
        assert_err!(claims);

        Ok(())
    }
}
