use axum::{async_trait, RequestPartsExt};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::{extract::Request, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;
use crate::ctx::Ctx;
use crate::{Error, Result};
use axum::body::Body;

use crate::web::AUTH_TOKEN;
pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>, 
    next: Next,
) -> Result<Response> {
    println!(">> MIDDLEWARE: mw_require_auth called!");
    ctx?;
    Ok(next.run(req).await)
}

// region --- CTx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!(">> EXTRACTOR: Ctx called!");
        // User the cookie to extract user_id
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        if auth_token == None {
            return Err(Error::AuthFailNoAuthTokenCookie);
        }

        let (user_id, _exp, _sign) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;

        Ok(Ctx::new(user_id))
    }
}

// endregion --- CTx Extractor

// Parse a token of format `user-[user_id].[expiration].[signature]`
// Returns (user_id, expiration, signature)

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id : u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}