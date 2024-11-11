use alloy::{consensus::TxEnvelope, primitives::Bytes, signers::local::PrivateKeySigner};
use eyre::Context;
use reqwest::Url;
use serde::Serialize;

use crate::utils::{envelope_to_raw_btyes, prepare_rpc_request, sign_request};

// https://github.com/chainbound/bolt/blob/lore/feat/holesky-launch/bolt-cli/src/commands/send.rs#L182
pub async fn send_bolt_request(
    target_slot: u64,
    tx: TxEnvelope,
    url: Url,
    wallet: &PrivateKeySigner,
) -> eyre::Result<()> {
    let tx_hashes = vec![*tx.tx_hash()];
    let raw_txs: Vec<Bytes> = vec![envelope_to_raw_btyes(&tx)];

    let request = prepare_rpc_request(
        "bolt_requestInclusion",
        serde_json::json!({
            "slot": target_slot,
            "txs": raw_txs,
        }),
    )
    .await;

    let signature = sign_request(tx_hashes, target_slot, wallet).await?;

    let response = reqwest::Client::new()
        .post(url)
        .header("x-bolt-signature", signature)
        .json(&request)
        .send()
        .await
        .wrap_err("failed to send constraints")?;

    let body = response
        .bytes()
        .await
        .wrap_err("failed to parse response")?;
    let body = String::from_utf8_lossy(&body);
    println!("Response: {}", body);

    Ok(())
}

// https://developers.ethgas.com/#post-api-inclusion_preconf-send
pub async fn send_ethgas_request(
    target_slot: u64,
    tx: TxEnvelope,
    base_url: Url,
) -> eyre::Result<()> {
    let url = base_url.join("/api/inclusion_preconf/send")?;

    let request = EthGasRequest {
        slot_number: target_slot,
        replacement_uuid: "01ab2371-84d6-459e-95e7-5edad485f282".to_string(),
        trxs: vec![EthGasTxRequest {
            tx: envelope_to_raw_btyes(&tx),
            can_revert: false,
        }],
    };

    let response = reqwest::Client::new()
        .post(url)
        .json(&request)
        .send()
        .await
        .wrap_err("failed to send constraints")?;

    let body = response
        .bytes()
        .await
        .wrap_err("failed to parse response")?;
    let body = String::from_utf8_lossy(&body);
    println!("Response: {}", body);

    Ok(())
}

// {
//     slotNumber: 114713,
//     replacementUuid: '01ab2371-84d6-459e-95e7-5edad485f282',
//     trxs: [{tx: '0x02f885827a6901843b9aca0084773594008252089412643b525cc34282ba84298d32bf2d094448f1c4019945746847617320496e636c7573696f6e20507265636f6e6673c001a0156d9b84193af432f32aef3976417dfca1f0d71f8e015ba8b3d68a11fe388a5ea059eefd7c77489551dfb04a887493617b83a4b78923b7592a992f0ed5c57d520a', canRevert: true},
//            {tx: '0x02f885827a6901843b9aca0084773594008252089412643b525cc34282ba84298d32bf2d094448f1c4019945746847617320496e636c7573696f6e20507265636f6e6673c001a0156d9b84193af432f32aef3976417dfca1f0d71f8e015ba8b3d68a11fe388a5ea059eefd7c77489551dfb04a887493617b83a4b78923b7592a992f0ed5c57d520a', canRevert: false}
//     ]
// }

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct EthGasRequest {
    slot_number: u64,
    replacement_uuid: String,
    trxs: Vec<EthGasTxRequest>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct EthGasTxRequest {
    tx: Bytes,
    can_revert: bool,
}

pub async fn send_luban_request(
    target_slot: u64,
    tx: TxEnvelope,
    url: Url,
    wallet: &PrivateKeySigner,
) -> eyre::Result<()> {
    let tx_hashes = vec![*tx.tx_hash()];
    let raw_txs: Vec<Bytes> = vec![envelope_to_raw_btyes(&tx)];

    let request = prepare_rpc_request(
        "luban_requestInclusion",
        serde_json::json!({
            "slot": target_slot,
            "txs": raw_txs,
        }),
    )
    .await;

    let signature = sign_request(tx_hashes, target_slot, wallet).await?;

    let response = reqwest::Client::new()
        .post(url)
        .header("x-bolt-signature", signature)
        .json(&request)
        .send()
        .await
        .wrap_err("failed to send constraints")?;

    let body = response
        .bytes()
        .await
        .wrap_err("failed to parse response")?;
    let body = String::from_utf8_lossy(&body);
    println!("Response: {}", body);

    Ok(())
}
