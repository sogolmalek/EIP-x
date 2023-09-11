use config::Config;
use consensus::database::Database;
use ethers::types::{Address, U256};
use eyre::{Report, Result};
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

// It looks like Checkpoints are sent to the Partial Data view and this is serialised with rmp_serde
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

// TODD: Implemnt these traits properly. They are mostly no ops right now.
impl Database for TemporaryDB {
    fn new(_config: &Config) -> Result<Self> {
        Ok(Self::default())
    }

    // @dev: We need to return a Vec<u8> here , as the return values need to match the traits implementation.
    // see consensus/database.rs

    fn load_checkpoint(&self) -> Result<Vec<u8>, Report> {
        // let db = TemporaryDB::new;
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
        let storage = PartialViewDataStorage {
            partial_view_data: HashMap::new(),
            address_queue: VecDeque::new(),
            max_queue_capacity,
            database,
        };


        Ok(storage)
    }

    // @todo: query and store
    pub fn query_and_store(&mut self, _address: Address) -> Result<()> {
        // Query and store data as before

        // After storing data, save the checkpoint to the database
        // self.save_checkpoint()?;
        unimplemented!();
    }

    fn load_checkpoint(&mut self) -> Result<CheckpointData, Report> {
        let checkpoint_bytes: Vec<u8> = self.database.load_checkpoint()?;
        let checkpoint: CheckpointData =
            rmp_serde::from_slice(&checkpoint_bytes).map_err(|e| eyre::eyre!(e.to_string()))?;

        Ok(checkpoint)
    }

    fn save_checkpoint(&self, checkpoint: CheckpointData) -> Result<()> {
        let mut buffer = Vec::new();
        checkpoint
            .serialize(&mut Serializer::new(&mut buffer))
            .unwrap();

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
    use serde_bytes::ByteBuf;

    #[test]
    fn test_temporary_db() -> Result<()> {
        let temp_db = TemporaryDB::new();

        // Test save_checkpoint
        let checkpoint_data = CheckpointData {
            field1: "Test".to_string(),
            field2: 42,
            binary_data: ByteBuf::from(vec![0, 1, 2, 3]),
        };
        let serialized = rmp_serde::to_vec(&checkpoint_data)?;
        temp_db.save_checkpoint(&serialized)?;

        // Test load_checkpoint
        let loaded = temp_db.load_checkpoint()?;
        assert_eq!(serialized, loaded);

        Ok(())
    }

    #[test]
    fn test_partial_view_data_storage() -> Result<()> {
        let temp_db = TemporaryDB::new();
        let mut storage = PartialViewDataStorage::new(10, temp_db)?;

        // Test save_checkpoint
        let checkpoint_data = CheckpointData {
            field1: "Test".to_string(),
            field2: 42,
            binary_data: ByteBuf::from(vec![0, 1, 2, 3]),
        };
        storage.save_checkpoint(checkpoint_data.clone())?;

        // Test load_checkpoint
        storage.load_checkpoint()?;
        let loaded = storage.database.load_checkpoint()?;
        let deserialized: CheckpointData = rmp_serde::from_slice(&loaded)?;
        assert_eq!(checkpoint_data, deserialized);

        Ok(())
    }
}
