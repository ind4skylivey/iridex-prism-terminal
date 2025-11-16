use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::{PrismError, PrismResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Option<String>,
    pub exp: usize,
    pub iat: Option<usize>,
}

pub fn validate(token: &str, secret: Option<&str>) -> PrismResult<Claims> {
    if let Some(secret) = secret {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )?;
        Ok(data.claims)
    } else {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(PrismError::new("invalid JWT format"));
        }
        let payload = URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|err| PrismError::new(format!("invalid JWT payload: {err}")))?;
        let claims: Claims =
            serde_json::from_slice(&payload).map_err(|err| PrismError::new(err.to_string()))?;
        enforce_exp(&claims)?;
        Ok(claims)
    }
}

pub fn issue(secret: &str, subject: Option<&str>, ttl: chrono::Duration) -> PrismResult<String> {
    if ttl.num_seconds() <= 0 {
        return Err(PrismError::new("JWT duration must be greater than zero"));
    }
    if secret.is_empty() {
        return Err(PrismError::new("JWT secret cannot be empty"));
    }
    let now = chrono::Utc::now();
    let expires = now + ttl;
    let exp =
        usize::try_from(expires.timestamp()).map_err(|_| PrismError::new("JWT exp overflow"))?;
    let iat = usize::try_from(now.timestamp()).map_err(|_| PrismError::new("JWT iat overflow"))?;
    let claims = Claims {
        sub: subject.map(|value| value.to_string()),
        exp,
        iat: Some(iat),
    };
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

fn enforce_exp(claims: &Claims) -> PrismResult<()> {
    let exp_time = chrono::DateTime::<chrono::Utc>::from_timestamp(claims.exp as i64, 0)
        .ok_or_else(|| PrismError::new("invalid JWT exp"))?;
    if chrono::Utc::now() > exp_time {
        return Err(PrismError::new("JWT token has expired"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_issue_and_validate() {
        let secret = "test-secret";
        let token = issue(secret, Some("tester"), chrono::Duration::seconds(60)).unwrap();
        let claims = validate(&token, Some(secret)).unwrap();
        assert_eq!(claims.sub.as_deref(), Some("tester"));
    }
}
