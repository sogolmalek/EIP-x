use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use ethers::types::{Address, Block, BlockTag, U256};
use helios::{client::ClientBuilder, config::networks::Network, Database};
use eyre::Result;
use serde::{Deserialize, Serialize}; // Import serde traits
use std::sync::Arc;
use tokio::sync::RwLock;

// Temporary in-memory database
#[derive(Default)]
struct TemporaryDB {
    checkpoint: Option<String>, // Use String to store JSON checkpoint
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
        if let Some(checkpoint) = &self.checkpoint {
            // Deserialize the JSON checkpoint
            let bytes = checkpoint.as_bytes();
            let deserialized_checkpoint: Vec<u8> = serde_json::from_slice(bytes)?;

            Ok(deserialized_checkpoint)
        } else {
            Err(eyre::eyre!("No checkpoint found in the database"))
        }
    }

    fn save_checkpoint(&self, checkpoint: Vec<u8>) -> Result<()> {
        // Serialize the checkpoint to JSON
        let serialized_checkpoint = serde_json::to_string(&checkpoint)?;

        // For a temporary in-memory database, we simply update the checkpoint in memory.
        self.checkpoint = Some(serialized_checkpoint);
        Ok(())
    }
}

#[derive(Default, Serialize, Deserialize)] // Add serialization and deserialization support
pub struct PartialViewDataStorage<D>
where
    D: Database,
{
    partial_view_data: HashMap<Address, U256>,
    address_queue: VecDeque<Address>,
    max_queue_capacity: usize,
    database: D,
}

impl<D> PartialViewDataStorage<D>
where
    D: Database,
{
    pub fn new(max_queue_capacity: usize, database: D) -> Result<Self> {
        let mut storage = PartialViewDataStorage {
            partial_view_data: HashMap::new(),
            address_queue: VecDeque::new(),
            max_queue_capacity,
            database,
        };

        // Load the checkpoint from the database during initialization
        storage.load_checkpoint()?;

        Ok(storage)
    }

    pub fn query_and_store(&mut self, address: Address) -> Result<()> {
        // Query and store data as before

        // After storing data, save the checkpoint to the database
        self.save_checkpoint()?;

        Ok(())
    }

    fn load_checkpoint(&mut self) -> Result<()> {
        let checkpoint: Vec<u8> = self.database.load_checkpoint()?;
        // No need for deserialization here since we'll be working with bytes

        Ok(())
    }

    fn save_checkpoint(&self) -> Result<()> {
        // Serialize the checkpoint to bytes
        let serialized_checkpoint = vec![0, 1, 2, 3]; // Replace with your serialization logic

        // Save the serialized checkpoint to the database
        self.database.save_checkpoint(serialized_checkpoint)?;

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
        .database(TemporaryDB::new())
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
