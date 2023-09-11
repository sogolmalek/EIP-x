use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// Placeholder for Axiom ZK usage
mod axiom_zk {
    use super::*;
    pub fn verify_proof(_proof: Vec<u8>) -> bool {
        // Replace this with actual Axiom ZK verification code
        todo!();
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
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");

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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;
    use tokio::runtime::Runtime;

    #[test]
    // fn test_verify_proof() {
    //     let proof = vec![1, 2, 3, 4, 5];
    //     assert_eq!(axiom_zk::verify_proof(proof), true);
    // }
    #[test]
    fn test_zk_proof_message_serialization() {
        let message = ZkProofMessage {
            proof: vec![1, 2, 3, 4, 5],
            block_number: 10,
        };

        let serialized = serde_json::to_vec(&message).unwrap();
        let deserialized: ZkProofMessage = serde_json::from_slice(&serialized).unwrap();

        assert_eq!(message.proof, deserialized.proof);
        assert_eq!(message.block_number, deserialized.block_number);
    }

    // #[test]
    // fn test_handle_connection() {
    //     let rt = Runtime::new().unwrap();
    //     rt.block_on(async {
    //         let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    //         let (mut socket, _) = listener.accept().await.unwrap();

    //         let message = ZkProofMessage {
    //             proof: vec![1, 2, 3, 4, 5],
    //             block_number: 10,
    //         };

    //         let serialized = serde_json::to_vec(&message).unwrap();
    //         socket.write_all(&serialized).await.unwrap();

    //         let handle = tokio::spawn(handle_connection(socket));
    //         handle.await.unwrap();
    //     });
    // }
}
