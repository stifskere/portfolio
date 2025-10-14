use std::env::var;
use std::future::ready;
use std::future::Ready;

use actix_web::dev::Payload;
use actix_web::FromRequest;
use actix_web::Error as ActixError;
use actix_web::HttpRequest;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use bcrypt::verify;


pub struct OptionalAuth(bool);

impl OptionalAuth {
    pub fn is_authorized(&self) -> bool {
        self.0
    }
}

impl FromRequest for OptionalAuth {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let Ok(env_user) = var("PORTFOLIO_USER") else {
            return ready(Ok(OptionalAuth(false)));
        };

        let Ok(env_password) = var("PORTFOLIO_PASSWORD") else {
            return ready(Ok(OptionalAuth(false)));
        };

        let authorized = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|auth| auth.strip_prefix("Basic "))
            .and_then(|encoded| BASE64_STANDARD.decode(encoded).ok())
            .and_then(|decoded| String::from_utf8(decoded).ok())
            .and_then(|creds| {
                let creds = creds.split_once(':');
                Some((creds?.0.to_string(), creds?.1.to_string()))
            })
            .map(|(user, pass)| user == env_user && verify(pass, &env_password).unwrap_or(false))
            .unwrap_or(false);

        ready(Ok(OptionalAuth(authorized)))
    }
}
