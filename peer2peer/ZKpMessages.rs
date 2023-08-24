use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Placeholder for Axiom ZK usage
mod axiom_zk {
    pub fn verify_proof(_proof: Vec<u8>) -> bool {
        // Replace this with actual Axiom ZK verification code
        true
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ZkProofMessage {
    proof: Vec<u8>,
    block_number: u64,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");

    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer).await {
        Ok(0) => println!("Connection closed by remote"),
        Ok(n) => {
            let message = &buffer[..n];

            if let Ok(zk_proof_message) = serde_json::from_slice::<ZkProofMessage>(message) {
                println!("Received ZK Proof Message: {:?}", zk_proof_message);

                // Verify the proof using Axiom ZK's verification function
                let proof_valid = axiom_zk::verify_proof(zk_proof_message.proof);

                if proof_valid {
                    // Handle the verified proof - perform actions here

                    // Echo back the received proof message
                    if let Err(e) = socket.write_all(message).await {
                        eprintln!("Failed to write to socket: {}", e);
                    }
                } else {
                    eprintln!("Proof verification failed");
                    // Handle verification failure - perform actions here
                }
            } else {
                eprintln!("Failed to deserialize ZK Proof Message");
            }
        }
        Err(e) => eprintln!("Failed to read from socket: {}", e),
    }
}
