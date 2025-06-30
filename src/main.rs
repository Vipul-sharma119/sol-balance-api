use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

mod api;
mod solana_client;
mod types;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/balance/:address", get(api::get_balance));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}