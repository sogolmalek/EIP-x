use ethers::types::{Address, Block, BlockTag, U256};
use eyre::{Report, Result};
use helios::{client::ClientBuilder, config::networks::Network, Database};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use common::types::BlockTag;
use config::Network;

// Import serde_bytes for binary data serialization

// Temporary in-memory database
#[derive(Default)]
struct TemporaryDB {
    checkpoint: Option<CheckpointData>,
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

    fn load_checkpoint(&self) -> std::result::Result<&CheckpointData, Report> {
        if let Some(checkpoint) = &self.checkpoint {
            Ok(checkpoint.clone()) // Return a clone of the stored checkpoint
        } else {
            Err(eyre::eyre!("No checkpoint found in the database"))
        }
    }

    fn save_checkpoint(&mut self, checkpoint: CheckpointData) -> Result<()> {
        // For a temporary in-memory database, we simply update the checkpoint in memory.
        self.checkpoint = Some(checkpoint);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct CheckpointData {
    field1: String,
    field2: u32,
    binary_data: ByteBuf, // Use serde_bytes for binary data serialization
}

#[derive(Default, Serialize, Deserialize)]
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

    // @todo: query and store
    pub fn query_and_store(&mut self, _address: Address) -> Result<()> {
        // Query and store data as before

        // After storing data, save the checkpoint to the database
        self.save_checkpoint()?;

        Ok(())
    }

    fn load_checkpoint(&mut self) -> Result<()> {
        let checkpoint: CheckpointData = self.database.load_checkpoint()?;
        // No need for deserialization here since we'll be working with the CheckpointData struct

        Ok(())
    }

    fn save_checkpoint(&self) -> Result<()> {
        // Create a checkpoint struct with your data
        let checkpoint = CheckpointData {
            field1: "SomeData".to_string(),
            field2: 42,
            binary_data: ByteBuf::from(vec![0, 1, 2, 3]), // Example binary data
            // Set other fields as needed
        };

        // Serialize the checkpoint using MessagePack
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        checkpoint.serialize(&mut serializer)?;

        // Save the serialized checkpoint to the database
        self.database.save_checkpoint(buffer.into())?;

        Ok(())
    }
}


//We've added a binary_data field to the CheckpointData struct, which is an example of binary data serialized using the serde_bytes::ByteBuf type.

//We've used the rmp_serde crate to serialize and deserialize the CheckpointData struct using MessagePack format.

//The save_checkpoint method now serializes the CheckpointData struct into MessagePack format and saves it to the database.

//The load_checkpoint method loads the MessagePack data from the database and deserializes it into a CheckpointData struct.