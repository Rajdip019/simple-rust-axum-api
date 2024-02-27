
use crate::model::ModelController;

pub use self::error::{Error, Result};

use axum::extract::{Path, Query};
use axum::{middleware, Router};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let route_apis = web::routes_tickets::routes(mc.clone())
            .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    let app = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api", route_apis)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

    // start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", &listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello_path))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// eg: http://localhost:8080/hello?name=Rajdeep
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    println!("handler_hello called!");
    Html(format!("Hello, <strong>{name}</strong>"))
}


// eg: http://localhost:8080/hello/Rajdeep
async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("handler_hello_path called!");
    Html(format!("Hello, <strong>{name}</strong>"))
}
