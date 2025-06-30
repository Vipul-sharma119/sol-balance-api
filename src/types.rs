use serde::Serialize;

#[derive(Serialize)]
pub struct BalanceResponse {
    pub lamports: u64,
    pub sol: f64,
}