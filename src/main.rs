use std::net::SocketAddr;

use aide::{openapi::{OpenApi, Info}, axum::{ApiRouter, IntoApiResponse}, redoc::Redoc};
use axum::{Router, Extension, Json};
use aide::axum::routing::get;


#[tokio::main]
async fn main() {
    let router = make_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await.unwrap();
}


fn make_router() -> Router {
    let mut api = OpenApi {
        info: Info {
            description: Some("Some API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let router = ApiRouter::new()
        .api_route("/ping", get(pong))
        .route("/redoc", Redoc::new("/api.json").axum_route())
        .route("/api.json", get(serve_api));

    let router = router.finish_api(&mut api);
    router
}

async fn pong() -> &'static str {
    "pong"
}

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}