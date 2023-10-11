use clap::Parser;
use common::utils::hex_str_to_bytes;
use dirs::home_dir;
use env_logger::Env;
use ethers::types::transaction;
use eyre::{eyre, Result};
use std::net::IpAddr;
use std::{
    path::PathBuf,
    process::exit,
    str::FromStr,
    sync::{Arc, Mutex},
};

use client::{Client, ClientBuilder};
use config::{CliConfig, Config};
use futures::executor::block_on;
use log::{error, info};
use ethers::{
    core::types::{Block, BlockId, Transaction, TransactionReceipt, H256, Address},
    providers::{Http, Middleware, Provider},
    signers::Wallet,
    // trie::{MerklePatriciaTrie, Trie},
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
//sogol addded: 
    // Initialize the Ethereum provider URL and address public key from environment variables
    let provider_url = std::env::var("PROVIDER_URI").unwrap_or_else(|_| "http://127.0.0.1:8545".to_string());
    // let public_key = std::env::var("ETHEREUM_ADDRESS_PUBLIC_KEY").expect("ETHEREUM_ADDRESS_PUBLIC_KEY not set");
    // Initialize the Ethereum provider URL from environment variable or use default
    let provider = Provider::<Http>::try_from(provider_url.clone())?;

    let block_number = 9751182; // Replace with the desired block number
    let blockwithtransactions=(provider.get_block_with_txs(block_number).await).unwrap().unwrap();
    let transactions = blockwithtransactions.transactions;
    // Fetch all transactions within the specified block
    //let transactions = fetch_all_transactions(&provider, block_number).await?;

    let addresses = transactions.iter()
        .filter_map(|tx| {
            let from = tx.from;
            let to = tx.to.unwrap_or_default();
            
                Some(to)
        })
        .collect::<Vec<_>>();
    println!(
        "Addresses: {:?}, Number of Transactions: {}",
        addresses,
        transactions.len()
    );
    
    let config = get_config();

    // Create the Helios client with the specified target addresses
    let mut client = match ClientBuilder::new().config(config).build() {
        Ok(client) => client,
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    };

    if let Err(err) = client.start().await {
        error!("{}", err);
        exit(1);
    }

    let provider = Provider::<Http>::try_from(
        provider_url,
    )?;

    let block_number = 9751182;
    let block = provider
        .get_block_with_txs(BlockId::Number(block_number.into()))
        .await?;

    let block = match block {
        Some(block) => block,
        None => return Err(eyre!("Block not found")),
    };

    let addresses = block
        .transactions
        .iter()
        .map(|tx| {
            let from = tx.from;
            let to = tx.to;
            vec![from, to.unwrap_or_default()]
        })
        .flatten()
        .collect::<Vec<_>>();

    println!(
        "Addresses: {:?}, State root: {:?}",
        addresses, block.state_root
    );
   
    let config = get_config();

    // Define your target addresses here
    // We shouldnt need this any, as we pass the addresses as optional flags in the cli
    // let target_addresses = vec![
    //     Address::from_str("0xYourTargetAddress1").unwrap(),
    //     Address::from_str("0xYourTargetAddress2").unwrap(),
    // ];


    // Create the Helios client with the specified target addresses
    let mut client = match ClientBuilder::new().config(config).build() {
        Ok(client) => client,
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    };

    if let Err(err) = client.start().await {
        error!("{}", err);
        exit(1);
    }

    register_shutdown_handler(client);
    std::future::pending().await
}

fn register_shutdown_handler(client: Client) {
    let client = Arc::new(client);
    let shutdown_counter = Arc::new(Mutex::new(0));

    ctrlc::set_handler(move || {
        let mut counter = shutdown_counter.lock().unwrap();
        *counter += 1;

        let counter_value = *counter;

        if counter_value == 3 {
            info!("forced shutdown");
            exit(0);
        }

        info!(
            "shutting down... press ctrl-c {} more times to force quit",
            3 - counter_value
        );

        if counter_value == 1 {
            let client = client.clone();
            std::thread::spawn(move || {
                block_on(client.shutdown());
                exit(0);
            });
        }
    })
    .expect("could not register shutdown handler");
}

fn get_config() -> Config {
    let cli = Cli::parse();

    let config_path = home_dir().unwrap().join(".helios/helios.toml");

    let cli_config = cli.as_cli_config();

    Config::from_file(&config_path, &cli.network, &cli_config)
}
//sogol added:
// Fetch the block data, including the state root
// let block_number_to_fetch = 12345; // Replace with the desired block number
// let block_data = fetch_block_data(&provider_url, block_number_to_fetch).await?;

// // Iterate through accounts and fetch their state roots
// for account_address in get_all_accounts(&block_data.state_root) {
//     let state_root = fetch_state_root(&block_data.state_root, &account_address).await?;
//     // Process or store the state root as needed
//     println!("Account: {:?}, State Root: {:?}", account_address, state_root);
// }

async fn fetch_all_transactions(provider:Provider<Http>, blocknumber:i32)->Vec<Transaction> {
    let blockwithtransactions=(provider.get_block_with_txs(12).await).unwrap().unwrap();
    let transactions = blockwithtransactions.transactions;
    return transactions;

}
#[derive(Parser)]
#[clap(version, about)]
/// Helios is a fast, secure, and portable light client for Ethereum
struct Cli {
    #[clap(short, long, default_value = "mainnet")]
    network: String,
    #[clap(short = 'b', long, env)]
    rpc_bind_ip: Option<IpAddr>,
    #[clap(short = 'p', long, env)]
    rpc_port: Option<u16>,
    #[clap(short = 'w', long, env)]
    checkpoint: Option<String>,
    #[clap(short, long, env)]
    execution_rpc: Option<String>,
    #[clap(short, long, env)]
    consensus_rpc: Option<String>,
    #[clap(short, long, env)]
    data_dir: Option<String>,
    #[clap(short = 'f', long, env)]
    fallback: Option<String>,
    #[clap(short = 'l', long, env)]
    load_external_fallback: bool,
    #[clap(short = 's', long, env)]
    strict_checkpoint_age: bool,
    #[clap(short = 'a', long, env)]
    target_addresses: Option<Vec<String>>,
}

impl Cli {
    fn as_cli_config(&self) -> CliConfig {
        let checkpoint = self
            .checkpoint
            .as_ref()
            .map(|c| hex_str_to_bytes(c).expect("invalid checkpoint"));
        CliConfig {
            checkpoint,
            execution_rpc: self.execution_rpc.clone(),
            consensus_rpc: self.consensus_rpc.clone(),
            data_dir: self.get_data_dir(),
            rpc_bind_ip: self.rpc_bind_ip,
            rpc_port: self.rpc_port,
            fallback: self.fallback.clone(),
            load_external_fallback: self.load_external_fallback,
            strict_checkpoint_age: self.strict_checkpoint_age,
            target_addresses: self.target_addresses.clone(),
        }
    }

    fn get_data_dir(&self) -> PathBuf {
        if let Some(dir) = &self.data_dir {
            PathBuf::from_str(dir).expect("cannot find data dir")
        } else {
            home_dir()
                .unwrap()
                .join(format!(".helios/data/{}", self.network))
        }
    }
}
