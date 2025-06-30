// This file contains the logic to interact with the Solana blockchain.

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub async fn fetch_balance(wallet_address: &str) -> Result<u64, String> {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let pubkey = wallet_address.parse().map_err(|e| format!("Invalid address: {e}"))?;
    
    client.get_balance(&pubkey).map_err(|e| format!("RPC error: {e}"))
}