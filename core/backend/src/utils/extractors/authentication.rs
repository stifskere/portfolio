use std::future::{ready, Ready};
use std::sync::OnceLock;

use actix_failwrap::ErrorResponse;
use actix_web::http::header::AUTHORIZATION;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::{Utc, Duration};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::Error as JwtError;
use rand::rand_core::OsError as OsRngError;
use rand::rngs::{OsRng, StdRng};
use rand::distr::{Alphanumeric, SampleString};
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::utils::application::context::AppContext;
use crate::utils::misc::error_transformer::json_transformer;

/// The authentication cookie key
pub const COOKIE_KEY: &str = "authentication";
/// How long until the authentication session expires.
pub const AUTH_EXPIRATION_HOURS: i64 = 3;

/// Holds any error that may occur during the authentication
/// process with `OptionalAuth`.
#[derive(ErrorResponse, Error, Debug)]
#[transform_response(json_transformer)]
pub enum OptionalAuthError {
    #[error("Couldn't load application context.")]
    MissingContext,

    #[error("Couldn't generate a valid JWT secret, {0:#}")]
    JwtSecret(#[from] OsRngError),

    #[error("Couldn't obtain an expiration date for the JWT.")]
    JwtExpiration,

    #[error("Couldn't encode JWT, {0:#}")]
    JwtEncoding(#[from] JwtError),

    #[error("Attempted to perform a failing cast between two numeric values.")]
    InvalidCast
}

/// The claims that the application JWT consists of.
///
/// The email is a filler and the expiration is
/// managed by the jwt crate.
#[derive(Serialize, Deserialize, Debug)]
struct OptionalAuthClaims {
    email: String,
    exp: usize
}

/// `OptionalAuth` is an Actix Web extractor
/// that handles optional basic authentication.
///
/// It reads the `Authentication` header, a cookie
/// whose name is defined in COOKIE_KEY and attempts
/// to authenticate, creating or storing the user
/// provided JWT.
///
/// In the case where the user is not authenticated,
/// `token` is set to `None`, but the request
/// proceeds normally. **Is it implementor's responsability
/// to check `OptionalAuth::is_authenticated()`.**
///
/// In the case of invalid credentials passed, the
/// authentication will be ignored and set unauthenticated.
///
/// see: https://www.rfc-editor.org/rfc/rfc9110.html
/// about ignoring user errors while authenticating.
pub struct OptionalAuth {
    token: Option<String>
}

impl OptionalAuth {
    /// Constructor for unauthenticated requests.
    ///
    /// Acts as a shortener to avoid ambiguity.
    #[inline]
    const fn unauthenticated() -> Self {
        Self {
            token: None
        }
    }

    /// Constructor for authenticated requests.
    ///
    /// Acts as a shortener to avoid ambiguity.
    #[inline]
    const fn authenticated(token: String) -> Self {
        Self {
            token: Some(token)
        }
    }

    /// If he user is authenticated this returns the
    /// issued JWT, otherwise None.
    ///
    /// The provided token is only for use within
    /// this structure.
    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }

    /// Returns wether the user is authenticated
    /// or not.
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }
}

impl FromRequest for OptionalAuth {
    type Error = OptionalAuthError;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(try_authenticate(req))
    }
}

/// Stores a randomly generated secret to be used
/// for JWT encrypting and decrypting.
///
/// This ensures the secret to be always the same.
///
/// XXX: The JWT may be rotated if needed.
fn get_jwt_secret() -> Result<&'static String, OsRngError> {
    static SECRET: OnceLock<String> = OnceLock::new();

    let mut rng = StdRng::try_from_rng(&mut OsRng)?;

    Ok(
        SECRET
            .get_or_init(|| {
                Alphanumeric
                    .sample_string(&mut rng, 32)
            })
    )
}

/// Error wrapper for the `FromRequest` middleware implementation for
/// `OptionalAuth`. Used to avoid needing `ready` right away and being
/// able to propagate errors within.
///
/// This is strictly called in the earlier mentioned implementation.
fn try_authenticate(req: &HttpRequest) -> Result<OptionalAuth, OptionalAuthError> {
    // Obtain the application wide context.
    //
    // If this fails 500 error is thrown.
    let app_context = req
        .app_data::<Data<AppContext>>()
        .ok_or(OptionalAuthError::MissingContext)?;

    let admin_email = app_context.environment().admin_email();
    let admin_password = app_context.environment().admin_password();

    // Attempt to obtain user-provided authentication
    // string from the Authentication header.
    //
    // see: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Authorization
    let header_credentials = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .map(ToString::to_string);

    // If the header contains basic authentication, try to authenticate with it.
    if let Some(credentials) = header_credentials.as_ref().and_then(|c| c.strip_prefix("Basic ")) {
        return try_authenticate_basic(credentials, admin_email, admin_password);
    }

    // If the header contains bearer authentication, try to authenticate with it.
    if let Some(credentials) = header_credentials.as_ref().and_then(|c| c.strip_prefix("Bearer ")) {
        return try_authenticate_bearer(credentials, admin_email);
    }

    // Attempt to obtain bearer token from a browser
    // provided cookie.
    let cookie_credentials = req
        .cookie(COOKIE_KEY)
        .map(|cookie| cookie
            .value()
            .to_owned()
        );

    // If there is a cookie, try to authenticate with it.
    if let Some(credentials) = cookie_credentials {
        return try_authenticate_bearer(&credentials, admin_email);
    }

    // Otherwise assume there is no authentication
    // and return an unauthenticated response.
    Ok(OptionalAuth::unauthenticated())
}

/// Takes a "basic" authentication token.
///
/// See: https://datatracker.ietf.org/doc/html/rfc7617
fn try_authenticate_basic(
    user_credentials: &str,
    admin_email: &str,
    admin_password: &str
) -> Result<OptionalAuth, OptionalAuthError> {
    // Decode the base64 string into bytes or return
    // an unauthenticated response if the user provided
    // an invalid base64 string.
    let Ok(decoded_bytes) = BASE64_STANDARD.decode(user_credentials)
    else {
        return Ok(OptionalAuth::unauthenticated());
    };

    // Convert the decoded base64 bytes into a manageable
    // string.
    let Ok(decoded_credentials) = String::from_utf8(decoded_bytes)
    else {
        return Ok(OptionalAuth::unauthenticated());
    };

    // Split the email and password, by standard they are
    // not urlencoded, so colons should be avoided beware
    // unwanted interferences.
    let Some((email_cred, password_cred)) = decoded_credentials.split_once(':')
    else {
        return Ok(OptionalAuth::unauthenticated());
    };

    if email_cred != admin_email || password_cred != admin_password {
        return Ok(OptionalAuth::unauthenticated());
    }

    // In the case the credentials are correct, store a JWT.

    // Get a valid timestamp for when the JWT should expire,
    // the expiration time is defined in `AUTH_EXPIRATION_HOURS`.
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(AUTH_EXPIRATION_HOURS))
        .ok_or(OptionalAuthError::JwtExpiration)?
        .timestamp();

    // Create some claims for the JWT.
    //
    // see: https://datatracker.ietf.org/doc/html/rfc7519
    let jwt_claims = OptionalAuthClaims {
        email: admin_email.to_string(),
        //             i64 -> usize
        exp: expiration.try_into()
            .map_err(|_| OptionalAuthError::InvalidCast)?
    };

    // Encode the JWT.
    let jwt = encode(
        &Header::default(),
        &jwt_claims,
        &EncodingKey::from_secret(get_jwt_secret()?.as_bytes())
    )?;

    Ok(OptionalAuth::authenticated(jwt))
}

/// Takes a "bearer" authentication token, i.e a JWT
/// if the decryption is successful and the email matches,
/// an authenticated response is returned.
fn try_authenticate_bearer(
    token: &str,
    admin_email: &str
) -> Result<OptionalAuth, OptionalAuthError> {
    // Decode the token into claims or return
    // unauthenticated if unsuccessful.
    let Ok(decode_result) = decode::<OptionalAuthClaims>(
        &token,
        &DecodingKey::from_secret(get_jwt_secret()?.as_bytes()),
        &Validation::new(Algorithm::HS256)
    )
    else {
        return Ok(OptionalAuth::unauthenticated());
    };

    // If the email contained by the JWT is not
    // the admin email take the JWT as invalid.
    if decode_result.claims.email != admin_email {
        return Ok(OptionalAuth::unauthenticated());
    }

    Ok(OptionalAuth::authenticated(token.to_string()))
}
