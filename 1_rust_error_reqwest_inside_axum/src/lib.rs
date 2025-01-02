use axum::{debug_handler, routing::any, Router};
use tower_service::Service;
use worker::{Context, Env, HttpRequest, Result};
use worker_macros::event;

fn router() -> Router {
    Router::new().route("/", any(index))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

#[debug_handler]
pub async fn index() -> &'static str {
    let _res = reqwest::get("https://httpbin.org/ip").await.unwrap();
    
    "Hello, World!"
}
