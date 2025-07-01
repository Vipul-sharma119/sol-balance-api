// This file defines the API endpoints. It exports a function `get_balance.
use axum::{
    extract::Path,
    response::Json,
};
use crate::solana_client::fetch_balance;
use crate::types::BalanceResponse;

pub async fn get_balance(Path(address): Path<String>) -> Json<BalanceResponse> {
    let lamports = fetch_balance(&address).await.unwrap_or(0);
    let sol = lamports as f64 / 1_000_000_000.0; // Convert lamports to SOL

    Json(BalanceResponse {
        lamports,
        sol,
    })
}
