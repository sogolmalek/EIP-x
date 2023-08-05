use std::{collections::HashMap, str::FromStr};

use ethers::types::{Address, Block, BlockTag, U256};
use helios::{client::ClientBuilder, config::networks::Network};
use eyre::Result;

use std::sync::Arc;
use tokio::sync::RwLock;

// Temporary in-memory database
#[derive(Default)]
struct TemporaryDB {
    checkpoint: Option<Vec<u8>>,
}

impl TemporaryDB {
    fn new() -> Self {
        Self::default()
    }
}

impl helios::Database for TemporaryDB {
    fn new(_config: &helios::Config) -> Result<Self> {
        Ok(Self::default())
    }

    fn load_checkpoint(&self) -> Result<Vec<u8>> {
        self.checkpoint
            .clone()
            .ok_or_else(|| eyre::eyre!("No checkpoint found in the database"))
    }

    fn save_checkpoint(&self, checkpoint: Vec<u8>) -> Result<()> {
        // For a temporary in-memory database, we simply update the checkpoint in memory.
        self.checkpoint = Some(checkpoint);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Replace this with your actual Helios RPC URL
    let helios_rpc_url = "https://helios-rpc-url.com";

    // Replace this with the address we want to query
    let address_str = "0x0000000000000000000000000000000000000000";
    let address = Address::from_str(address_str)?;

    // Create a new Helios client with the temporary in-memory database
    let mut client = ClientBuilder::new()
        .network(Network::MAINNET)
        .execution_rpc(helios_rpc_url)
        .database(TemporaryDB::new()) // Use the temporary database implementation
        .build()?;

    // Start the client
    client.start().await?;

    // Query the latest block number
    let block_number = client.get_block_number().await?;
    println!("Latest block number: {}", block_number);

    // Query the balance of the specified address at the latest block
    let balance = client.get_balance(&address, BlockTag::Latest).await?;
    println!(
        "Balance of {}: {}",
        address_str,
        ethers::utils::format_ether(balance)
    );

    // Query the latest block
    let latest_block: Option<Block> = client.get_block(BlockTag::Latest).await?;
    if let Some(block) = latest_block {
        println!("Latest block details: {:?}", block);
    } else {
        println!("Latest block not found");
    }

    Ok(())
}
