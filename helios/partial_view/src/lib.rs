use client::ClientBuilder;
use common::types::BlockTag;
use config::networks::Network;
use consensus::database::Database;
// use config::Network;
use config::Config;
use ethers::types::{Address, U256};
use eyre::{Report, Result};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
// Import serde_bytes for binary data serialization

#[derive(Serialize, Deserialize)]
pub struct CheckpointData {
    field1: String,
    field2: u32,
    binary_data: ByteBuf, // Use serde_bytes for binary data serialization
}

// Temporary in-memory database
#[derive(Default)]
struct TemporaryDB {
    checkpoint: RefCell<Option<CheckpointData>>,
}

// @dev Do we need this ?
impl TemporaryDB {
    fn new() -> Self {
        Self::default()
    }
}

impl Database for TemporaryDB {
    fn new(_config: &Config) -> Result<Self> {
        Ok(Self::default())
    }

    // @dev: We need to return a Vec<u8> here , as the return values need to match the traits implementation.
    // see consensus/database.rs

    fn load_checkpoint(&self) -> Result<Vec<u8>, Report> {
        if let Some(checkpoint) = self.checkpoint.borrow().as_ref() {
            let serialized =
                rmp_serde::to_vec(checkpoint).map_err(|e| eyre::eyre!(e.to_string()))?;
            Ok(serialized)
        } else {
            Err(eyre::eyre!("No checkpoint found in the database"))
        }
    }
    fn save_checkpoint(&self, checkpoint: &[u8]) -> Result<()> {
        let checkpoint_data =
            rmp_serde::from_slice(checkpoint).map_err(|e| eyre::eyre!(e.to_string()))?;

        // For a temporary in-memory database, we simply update the checkpoint in memory.
        *self.checkpoint.borrow_mut() = Some(checkpoint_data);
        Ok(())
    }
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
        // self.save_checkpoint()?;
        unimplemented!();
    }

    fn load_checkpoint(&mut self) -> Result<()> {
        let checkpoint_bytes: Vec<u8> = self.database.load_checkpoint()?;
        let checkpoint: CheckpointData =
            rmp_serde::from_slice(&checkpoint_bytes).map_err(|e| eyre::eyre!(e.to_string()))?;
        Ok(())
    }

    fn save_checkpoint(&self, checkpoint: &[u8]) -> Result<()> {
        // Create a checkpoint struct with your data
        // let checkpoint = CheckpointData {
        //     field1: "SomeData".to_string(),
        //     field2: 42,
        //     binary_data: ByteBuf::from(vec![0, 1, 2, 3]), // Example binary data
        //                                                   // Set other fields as needed
        // };
        let checkpoint_data =
            rmp_serde::from_slice(checkpoint).map_err(|e| eyre::eyre!(e.to_string()))?;

        // Serialize the checkpoint using MessagePack
        let mut buffer = Vec::new();
        let mut serializer = Serializer::new(&mut buffer);
        checkpoint.serialize(&mut serializer)?;

        // Save the serialized checkpoint to the database
        self.database.save_checkpoint(&buffer)?;

        Ok(())
    }
}

//We've added a binary_data field to the CheckpointData struct, which is an example of binary data serialized using the serde_bytes::ByteBuf type.

//We've used the rmp_serde crate to serialize and deserialize the CheckpointData struct using MessagePack format.

//The save_checkpoint method now serializes the CheckpointData struct into MessagePack format and saves it to the database.

//The load_checkpoint method loads the MessagePack data from the database and deserializes it into a CheckpointData struct.
#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_temporary_db() -> Result<()> {
        let mut db = TemporaryDB::new();

        // Test save_checkpoint
        let checkpoint_data = CheckpointData {
            field1: "Test".to_string(),
            field2: 123,
            binary_data: ByteBuf::from(vec![0, 1, 2, 3]),
        };
        db.save_checkpoint(checkpoint_data.clone())?;

        // Test load_checkpoint
        let loaded_checkpoint = db.load_checkpoint()?;
        assert_eq!(loaded_checkpoint.field1, checkpoint_data.field1);
        assert_eq!(loaded_checkpoint.field2, checkpoint_data.field2);
        assert_eq!(loaded_checkpoint.binary_data, checkpoint_data.binary_data);

        Ok(())
    }

    #[tokio::test]
    async fn test_partial_view_data_storage() -> Result<()> {
        let db = TemporaryDB::new();
        let mut storage = PartialViewDataStorage::new(10, db)?;

        // Test save_checkpoint
        let checkpoint_data = CheckpointData {
            field1: "Test".to_string(),
            field2: 123,
            binary_data: ByteBuf::from(vec![0, 1, 2, 3]),
        };
        storage.save_checkpoint()?;

        // Test load_checkpoint
        let loaded_checkpoint = storage.load_checkpoint()?;
        assert_eq!(loaded_checkpoint.field1, checkpoint_data.field1);
        assert_eq!(loaded_checkpoint.field2, checkpoint_data.field2);
        assert_eq!(loaded_checkpoint.binary_data, checkpoint_data.binary_data);

        Ok(())
    }
}
