use ethers::providers::{Alchemy, Middleware, Provider};
use ethers::types::{Address, U256};
use eyre::Result;
use helios::db::Database;
use helios::rpc::Rpc;
// main.rs

mod mod light_client_protocol;
 // Include the custom protocol from the separate file

use light_client_protocol::LightClientMessage; // Import the LightClientMessage enum for use in the LightClient

#[tokio::main]
async fn main() -> Result<()> {
    // Replace with your Alchemy Ethereum endpoint URL and Helios RPC URL
    let alchemy_endpoint = "FE1Fd3x7PlqkZYxMqQpP3orTaf1dsmG4";
    let helios_rpc_url = "http://your-helios-rpc-url:8545";

    // Initialize the Ethereum provider with Alchemy
    let alchemy = Alchemy::new(alchemy_endpoint).expect("Failed to initialize Alchemy provider");
    let provider = Provider::<Alchemy>::new(alchemy);

    // Initialize Helios client
    let helios_rpc = Rpc::new(helios_rpc_url).expect("Failed to initialize Helios RPC");
    let helios_client = helios::Client::new(helios_rpc);

    // Example query functions
    let address = Address::from_str("0x7a250d5630b4cf539739df2c5dacb4c659f2488d")
        .expect("Failed to parse address");

    // Get the latest block number using Alchemy
    let latest_block_number = get_latest_block_number(&provider).await?;
    println!("Latest block number: {}", latest_block_number);

    // Get the balance of an address using Helios
    let balance = get_balance(&helios_client, address).await?;
    println!("Balance of {}: {} ETH", address, balance);

    Ok(())
}

async fn get_latest_block_number<M: Middleware>(provider: &Provider<M>) -> Result<u64> {
    let block = provider.get_block("latest").await?;
    Ok(block.number.unwrap().as_u64())
}

async fn get_balance<D: Database>(client: &helios::Client<D>, address: Address) -> Result<f64> {
    let balance = client
        .eth_balance(address, None)
        .await?
        .as_u64() as f64 / 1_000_000_000_000_000_000.0;
    Ok(balance)
}
