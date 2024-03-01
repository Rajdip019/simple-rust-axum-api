use crate::model::ModelController;

pub use self::error::{Error, Result};

use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
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
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", &listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn main_response_mapper(
    uri: Uri,
    req_method: Method,
    res: Response
) -> Response {
    let uuid = Uuid::new_v4();

    // Get the eventual response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|e| e.client_status_and_error());

    // -- If client error, build the new response
    let error_response = client_status_error.as_ref().map(|(status, client_error)| {
        let client_error_body = json!({"error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
            }
        });

        println!(">> Client Error: {:?}", client_error_body);
        (*status, Json(client_error_body)).into_response()
    });

    println!(">> Server Log line - {uuid} - Error: {error:?}", uuid = uuid, error = client_status_error);

    // Build and log the request log line
    let client_error  = client_status_error.unzip().1;
    log::log_request(
        uuid.to_string(),
        req_method,
        uri,
        service_error.cloned(),
        client_error,
    ).await.unwrap(); 

    println!();
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/", get(handler_hello))
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
    Html(format!("Hello, <strong>{name} this is auto deployed</strong>"))
}

// eg: http://localhost:8080/hello/Rajdeep
async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("handler_hello_path called!");
    Html(format!("Hello, <strong>{name}</strong>"))
}


