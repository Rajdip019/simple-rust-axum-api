
use crate::web;
use serde::Deserialize;
use tower_cookies::Cookie;
use tower_cookies::Cookies;
use axum::routing::post;
use serde_json::json;
use serde_json::Value;
use axum::{Json, Router};
use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!(">> HANDLER: api_login called!");
    if payload.username != "rajdeep" || payload.pwd != "123" {
        return Err(Error::LoginFail);
    }

    // test cookie
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // Create Success body
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}