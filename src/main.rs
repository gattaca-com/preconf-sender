mod beacon;
mod protocols;
mod utils;

use alloy::{
    consensus::{Transaction, TxEnvelope},
    eips::eip2718::Decodable2718,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Bytes, U256},
    providers::{ProviderBuilder, WalletProvider},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use beacon::BeaconClient;
use clap::{Parser, ValueEnum};
use eyre::bail;
use protocols::{send_bolt_request, send_ethgas_request, send_luban_request};
use reqwest::Url;

#[derive(Parser, Debug)]
#[command(name = "preconf-sender")]
pub struct Args {
    #[arg(long("execution"), short)]
    exeuction_url: Url,

    #[arg(long("preconfer"))]
    preconfer_url: Url,

    #[arg(long("beacon"), short)]
    beacon_url: Url,

    /// Transaction to send, will be signed by the wallet
    #[arg(long)]
    tx: Option<Bytes>,

    /// Send a transfer to self
    #[arg(long)]
    random: bool,

    #[arg(long)]
    private_key: String,

    #[arg(long)]
    protocol: Protocol,
}

#[derive(Debug, ValueEnum, Clone)]
enum Protocol {
    Bolt,
    Ethgas,
    Luban,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let private_key: PrivateKeySigner = args.private_key.parse()?;
    println!("Sending tx from: {}", private_key.address());

    let wallet = EthereumWallet::from(private_key.clone());

    let beacon_client = BeaconClient::new(args.beacon_url);

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(args.exeuction_url);

    let tx_request = if args.random {
        TransactionRequest::default()
            .with_to(provider.default_signer_address())
            .with_gas_limit(21_000)
            .value(U256::from(1))
    } else if let Some(tx) = args.tx {
        let envelope = TxEnvelope::network_decode(&mut tx.as_ref())?;
        let to = envelope.to().expect("missing TO field");
        let input = envelope.input().clone();
        let value = envelope.value();

        TransactionRequest::default()
            .with_to(to)
            .with_input(input)
            .with_value(value)
    } else {
        bail!("Either specify --tx OR set --random")
    };

    let filled = provider.fill(tx_request).await?;
    let tx = filled.as_envelope().unwrap().clone();

    let head_slot = beacon_client.head_slot().await?;
    let next_slot = head_slot + 1;

    println!(
        "Sending preconf request to {:?} tx: {} slot: {}",
        args.protocol,
        tx.tx_hash(),
        next_slot
    );

    match args.protocol {
        Protocol::Bolt => {
            send_bolt_request(next_slot, tx, args.preconfer_url, &private_key).await?;
        }
        Protocol::Ethgas => {
            send_ethgas_request(next_slot, tx, args.preconfer_url).await?;
        }
        Protocol::Luban => {
            send_luban_request(next_slot, tx, args.preconfer_url, &private_key).await?;
        }
    }

    Ok(())
}
