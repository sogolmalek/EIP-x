use ethers::providers::{Alchemy, Provider};
use ethers::types::Address;
use eyre::Result;
use helios::db::{Database, InMemoryDatabase};
use helios::rpc::Rpc;
use std::collections::HashMap;
use bls12_381::{PublicKey, Signature, G1Affine, Fq12, Fq6, G2Affine, G2Projective};
use std::convert::TryFrom;

// Define a function to verify BLS signature
fn verify_signature(
    public_key: &PublicKey<G2Affine>,
    message: &[u8],
    signature: &Signature<G1Affine>,
) -> bool {
    // Verify the BLS signature
    public_key.verify::<Fq6, Fq12>(&message, signature)
}

// Define a cache struct to hold the cached data
struct Cache {
    cache: HashMap<Address, f64>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            cache: HashMap::new(),
        }
    }

    fn get(&self, address: &Address) -> Option<f64> {
        self.cache.get(address).cloned()
    }

    fn insert(&mut self, address: Address, balance: f64) {
        self.cache.insert(address, balance);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Replace with your Helios RPC URL
    let helios_rpc_url = "https://goerli-light.eth.linkpool.io";

    // Initialize the Ethereum provider with Alchemy
    let alchemy = Alchemy::new(helios_rpc_url).expect("Failed to initialize Alchemy provider");
    let provider = Provider::<Alchemy>::new(alchemy);

    // Initialize Helios client
    let helios_rpc = Rpc::new(helios_rpc_url).expect("Failed to initialize Helios RPC");
    let helios_client = helios::Client::new(helios_rpc);

    // Initialize the cache
    let mut cache = Cache::new();

    // Initialize the in-memory database
    let database = InMemoryDatabase::new();

    // Example Ethereum address to query
    let address = Address::from_str("0x0000000000000000000000000000000000000000")
        .expect("Failed to parse address");

    // Query the balance and store it in the partial view data
    query_balance(&helios_client, &database, &mut cache, address).await?;

    // Get the balance from the partial view data
    let balance = get_balance(&database, address);
    println!("Balance of {}: {} ETH", address, balance);

    Ok(())
}

async fn query_balance<D: Database>(
    client: &helios::Client<D>,
    database: &D,
    cache: &mut Cache,
    address: Address,
) -> Result<()> {
    // Check if the balance is already in the cache
    if let Some(balance) = cache.get(&address) {
        // If in cache, no need to query, just store it in the database
        database.save_checkpoint(address.to_string(), balance.to_string())?;
        return Ok(());
    }

    // Fetch the balance from the Helios client
    let balance = client
        .eth_balance(address, None)
        .await?
        .as_u64() as f64 / 1_000_000_000_000_000_000.0;

    // Insert the balance into the cache
    cache.insert(address, balance);

    // Store the balance in the partial view data (in-memory database)
    database.save_checkpoint(address.to_string(), balance.to_string())?;

    Ok(())
}

fn get_balance<D: Database>(database: &D, address: Address) -> f64 {
    // Try to get the balance from the cache first
    if let Some(balance) = database.load_checkpoint(address.to_string()) {
        return balance.parse().unwrap_or(0.0);
    }

    // If not in cache, return 0.0 as default
    0.0
}
