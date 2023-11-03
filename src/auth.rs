use crate::app_state::AppState;
use crate::user::{self, User, UserId};
use askama_axum::{IntoResponse, Response};
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::{async_trait, Json, RequestPartsExt, TypedHeader};
use headers::authorization::Bearer;
use headers::Authorization;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool, Sqlite};
use std::fmt::{self, Display};
use std::sync::Arc;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};

const USER_COOKIE: &str = "_fq_user";

const SECRET: &str = "secret";
// let SECRET = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

pub static KEYS: Lazy<Keys> = Lazy::new(|| Keys::new(SECRET.as_bytes()));

pub fn make_user_token(user: &User) -> jsonwebtoken::errors::Result<String> {
    let claims = Claims {
        sub: user.id,
        name: user.name.clone(),
        exp: 2_000_000_000,
    };
    encode(&Header::default(), &claims, &KEYS.encoding)
}

pub fn make_user_cookie(token: String) -> Cookie<'static> {
    Cookie::build(USER_COOKIE, token)
        .same_site(SameSite::Lax)
        .path("/")
        .finish()
}

fn decode_user_token(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(token, &KEYS.decoding, &Validation::default()).map(|data| data.claims)
}

pub fn decode_user_cookie(cookies: &Cookies) -> Option<User> {
    cookies
        .get(USER_COOKIE)
        .and_then(|cookie| decode_user_token(cookie.value()).ok())
        .map(User::from)
}

pub async fn authorize(
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
) -> Result<Json<AuthBody>, AuthError> {
    match cookies.get(USER_COOKIE) {
        None => {
            let user_id = user::create_user(&state.pool)
                .await
                .map_err(|_| AuthError::TokenCreation)?
                .last_insert_rowid();

            let user = User {
                id: user_id,
                name: user::DEFAULT_USERNAME.to_string(),
            };

            let token = make_user_token(&user).map_err(|_| AuthError::TokenCreation)?;
            let cookie = make_user_cookie(token.clone());
            cookies.add(cookie);

            Ok(Json(AuthBody::new(token)))
        }
        Some(cookie) => {
            let token = cookie.value().to_string();
            Ok(Json(AuthBody::new(token)))
        }
    }
}

pub async fn protected(claims: Claims) -> Result<String, AuthError> {
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub name: String,
    pub exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sub: {}", self.sub)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    token: String,
    token_type: String,
}

impl AuthBody {
    fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
