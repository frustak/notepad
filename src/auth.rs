use crate::{config, models::account::Account};
use chrono::Utc;
use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey};
use rocket::{
    http::{Cookie, Cookies},
    request::{FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    /// Expiration timestamp (UTC)
    pub exp: i64,
    pub account_id: i32,
    pub username: String,
}

impl Auth {
    /// Encode jwt token and return it
    pub fn token(&self, secret: &[u8]) -> String {
        jwt::encode(
            &jwt::Header::default(),
            self,
            &EncodingKey::from_secret(secret),
        )
        .expect("Error encoding JWT")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // try to authenticate account, if failed forward to next matching route
        if let Some(auth) = extract_auth_from_request(request, config::SECRET.as_ref()) {
            Outcome::Success(auth)
        } else {
            Outcome::Forward(())
        }
    }
}

/// Get authentication token from private cookies then decode it
fn extract_auth_from_request(request: &Request, secret: &[u8]) -> Option<Auth> {
    request
        .cookies()
        .get_private(config::TOKEN)
        .map(|cookie| cookie.value().to_string())
        .and_then(|token| decode_token(&token, secret))
}

/// Decode token into `Auth` struct. If any error is encountered, log it and return None.
fn decode_token(token: &str, secret: &[u8]) -> Option<Auth> {
    use jwt::{Algorithm, Validation};

    jwt::decode(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )
    .ok()
    .map(|token_data| token_data.claims)
}

/// Set authentication encoded token to cookies
pub fn set_auth_cookie(account: Account, mut cookies: Cookies) {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("Invalid timestamp")
        .timestamp();

    let auth = Auth {
        exp: expiration,
        username: account.username,
        account_id: account.account_id,
    };

    let token = auth.token(config::SECRET.as_ref());
    let auth_cookie = Cookie::new(config::TOKEN, token);
    cookies.add_private(auth_cookie);
}
