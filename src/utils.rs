use alloy::eips::eip2718::Encodable2718;
use alloy::{
    consensus::TxEnvelope,
    primitives::{keccak256, Bytes, B256},
    signers::{local::PrivateKeySigner, Signer},
};
use serde_json::Value;

// https://github.com/chainbound/bolt/blob/3173638f920b215284a63f55d3c8bc22210410a3/bolt-cli/src/commands/send.rs#L217C1-L233C2
pub async fn sign_request(
    tx_hashes: Vec<B256>,
    target_slot: u64,
    wallet: &PrivateKeySigner,
) -> eyre::Result<String> {
    let digest = {
        let mut data = Vec::new();
        let hashes = tx_hashes
            .iter()
            .map(|hash| hash.as_slice())
            .collect::<Vec<_>>()
            .concat();
        data.extend_from_slice(&hashes);
        data.extend_from_slice(target_slot.to_le_bytes().as_slice());
        keccak256(data)
    };

    let signature = alloy::hex::encode(wallet.sign_hash(&digest).await?.as_bytes());

    Ok(format!("{}:0x{}", wallet.address(), signature))
}

pub async fn prepare_rpc_request(method: &str, params: Value) -> Value {
    serde_json::json!({
        "id": "1",
        "jsonrpc": "2.0",
        "method": method,
        "params": vec![params],
    })
}

pub fn envelope_to_raw_bytes(tx: &TxEnvelope) -> Bytes {
    let mut encoded = Vec::new();
    tx.network_encode(&mut encoded);
    encoded.into()
}
