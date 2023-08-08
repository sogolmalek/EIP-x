// light_client_protocol.rs

use ethers::types::{Address, U256};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightClientMessage {
    RequestBalance(Address),
    ResponseBalance(Address, U256),
    // Add more message types as needed
}

