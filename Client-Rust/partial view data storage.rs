//In this implementation, we have created a PartialViewDataStorage struct that uses the Helios client with a temporary
// in-memory database to store and query partial view data (in this case, the balance of Ethereum addresses).
// The query_balance function queries the balance of an address at the latest block and stores 
//it in the partial_view_data map. The get_balance function allows querying the balance from the partial view data.
// need to replace "https://goerli-light.eth.linkpool.io" with the actual Helios RPC URL and "0x0000000000000000000000000000000000000000"
//with the Ethereum address  want to query. This implementation uses the Goerli testnet,


use std::collections::HashMap;
use std::str::FromStr;

use ethers::types::{Address, BlockTag, U256};
use helios::{client::ClientBuilder, config::networks::Network};
use eyre::Result;

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

// Partial View Data Storage
struct PartialViewDataStorage {
    client: helios::Client<TemporaryDB>,
    partial_view_data: HashMap<Address, U256>, // Map address to their partial view data (balance)
}

impl PartialViewDataStorage {
    // Create a new Partial View Data Storage
    async fn new(helios_rpc_url: &str) -> Result<Self> {
        // Create a new Helios client with the temporary in-memory database
        let client = ClientBuilder::new()
            .network(Network::GOERLI) // Replace with the desired testnet
            .execution_rpc(helios_rpc_url)
            .database(TemporaryDB::new())
            .build()?;

        Ok(Self {
            client,
            partial_view_data: HashMap::new(),
        })
    }

    // Query balance of an address and store it in the partial view data
    async fn query_balance(&mut self, address_str: &str) -> Result<()> {
        let address = Address::from_str(address_str)?;

        // Query the balance of the address at the latest block
        let balance = self.client.get_balance(&address, BlockTag::Latest).await?;

        // Store the balance in the partial view data
        self.partial_view_data.insert(address, balance);

        Ok(())
    }

    // Get the balance from the partial view data
    fn get_balance(&self, address_str: &str) -> Option<U256> {
        let address = Address::from_str(address_str).ok()?;
        self.partial_view_data.get(&address).cloned()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Replace this with your actual Helios RPC URL
    let helios_rpc_url = "https://goerli-light.eth.linkpool.io";

    let mut storage = PartialViewDataStorage::new(helios_rpc_url).await?;

    // Replace this with the address you want to query
    let address_str = "0x0000000000000000000000000000000000000000";
    storage.query_balance(address_str).await?;

    // Query the balance from the partial view data and print it
    if let Some(balance) = storage.get_balance(address_str) {
        println!("Balance of {}: {}", address_str, ethers::utils::format_ether(balance));
    } else {
        println!("Balance not found for the address");
    }

    Ok(())
}
