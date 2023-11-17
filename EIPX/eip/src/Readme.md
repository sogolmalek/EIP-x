
This is a single page that aims to explain how to run a test for our EIP-x POC

## Caviat: 
EIP-X is currently in unstable alpha, and __should not be used in production yet__. If you run into any bugs while using EIp-x, please file an Issue!

## Pre-queries: 

Rust installation
```bash
sudo pacman -S rustup
rustup install stable
``` 
Go installation

## Install dependencies: 

Rust toolchain => nightly-2023-04-24
Trin version: 349d3f4
Golang version: go1.21.4

Note: If you use a VPN, you should disable it before running Trin.

Install dependencies (Ubuntu/Debian):

```bash
apt install libssl-dev librocksdb-dev libclang-dev pkg-config build-essential

git clone https://github.com/sogolmalek/EIP-x.git
cd trin

# Build
cargo build --workspace

# Run
cargo run
``` 
## State verification Flow:

## Step 1: 

Fristly, we need to run the light client poc with server.rs submodul. this code involves functions subscribing to the mainnet events 

```rust 
  loop {
        let current_block = client.get_block_number().await.unwrap();
        if current_block <= last_processed_block {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let to = format!(
                "0x{:040x}",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            );

```

Next step it will  generates the witness of each new block on chain: 
```rust 
let witness = StateUpdateWitness::<Fr>::build(
            client.clone(),
            PROVIDER_URL,
            last_processed_block,
            None,
        )
        .await?;

        let Some(witness) = witness else {
                continue;
        };
```

finally the following snippet will generate the zkp of the block witness: 

```rust 
 let proof = circuit.prove(keys.as_ref().unwrap())?;

 ```

 The generated proof will be stored on server disk. once the zk proof is generated this code will verify it as well.

```rust 
 let file_path = last_processed_block.to_string() + ".bin";

        let mut file =  File::create(file_path).unwrap();
    
        match file.write_all(&proof) {
            Ok(_) => println!("Data written to file successfully"),
            Err(e) => eprintln!("Error writing to file: {}", e),
        }
 ```

## Step 2: 

To interact iwth trin at this stage, we implemented a local node that is connceted to Portal netwrok. This node recieves the 
ZKP messages.

The generated ZKp will be stored on the local node 

```rust
 let result1: bool = client
        .store(content_key.clone(), content_item.clone())
        .await
        .unwrap();
```
 these zk proofs will be next propagated to all participant nodes across p2p trin netwrok : 
```rust
   let result2 = client
        .gossip(content_key.clone(), content_item.clone())
        .await
        .unwrap();

```

Final step is to verify the zk proof recieved by each node participated in trin network :

```rust
 StateUpdateCircuit::verify(&proof, &public_inputs, keys.as_ref().unwrap())?;
        loop {
            // Don't want to wait to display 1st log, but a bug seems to skip the first wait, so put
            // this wait at the top. Otherwise, we get two log lines immediately on startup.
            heart_interval.tick().await;

            let storage_log = network.overlay.store.read().get_summary_info();
            let message_log = network.overlay.get_message_summary();
            let utp_log = network.overlay.get_utp_summary();
            info!("reports~ data: {storage_log}; msgs: {message_log}");
            info!("reports~ utp: {utp_log}");
        }
```
