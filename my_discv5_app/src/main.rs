use async_std::task;
use async_std::stream::StreamExt;
use discv5::{NodeDiscovery, NodeRecord, CombinedKey, CombinedKeyExt, Config, Discv5Error};
use std::net::{SocketAddr, Ipv4Addr};
use std::time::Duration;
use rand::Rng;
use rlp::{self, RlpStream};
use rlp_derive::{Decodable, Encodable};
use rlpx::{SecioKey, Endpoint, Protocol, SecioCodec, RemoteId, Frame};
use discv5::enr::{CombinedKey, Enr};


// Define a new enum for different message types, including the ZKP message
#[derive(Debug, Clone, PartialEq, Eq, Encodable, Decodable)]
enum MessageType {
    FindNodeRequest(Vec<u8>),
    ZkpMessage(vec<u8>),
    // Add other message types as needed
}

// Function to send a FindNode request with an appended ZKP message
async fn send_findnode_request(
    node_discovery: &NodeDiscovery,
    node: &NodeRecord,
    zkp_message: &[u8],
) -> Result<(), Discv5Error> {
    let request_id: [u8; 8] = rand::random();
    let distances = vec![0, 1, 2];

    let request = node_discovery.findnode(request_id, distances)?;
    let address = node.udp_socket().unwrap().local_addr().unwrap();
    let request_with_payload = MessageType::FindNodeRequest((request, zkp_message.to_vec()));

    send_message(&address, &request_with_payload).await
}

// Function to send a generic message over RLPx
async fn send_message(address: &SocketAddr, message: &MessageType) -> Result<(), Discv5Error> {
    // Encode the message using RLP
    let encoded_message = rlp::encode(message);

    // Set up RLPx connection
    let mut endpoint = Endpoint::new();
    endpoint.set_id(RemoteId::default());
    endpoint.set_key(SecioKey::new_temp().unwrap());

    // Simulate RLPx communication by sending the encoded message
    let frame = Frame::Data(message.encode());
    endpoint.write(frame).unwrap();

    Ok(())
}

// Function to simulate RLPx session setup
fn setup_rlp_session() {
    // Implement RLPx session setup logic here
    println!("RLPx session setup completed");
}

// Main function
fn main() {
    task::block_on(async {
        // Generate your local ENR and configure Node Discovery
        let local_key = CombinedKey::generate_secp256k1();
        let local_enr = local_key.generate_enr().unwrap();

        let config = Config {
            local_key,
            local_peer_id: local_enr.node_id(),
            listen_address: Some("0.0.0.0:9000".parse::<SocketAddr>().unwrap()),
            bootnodes: vec![
                // Replace with actual bootnodes
                SocketAddr::new(Ipv4Addr::new(1, 2, 3, 4), 30303),
            ],
            request_timeout: Duration::from_secs(5),
            max_request_retries: 3,
            ..Config::default()
        };

        let mut node_discovery = NodeDiscovery::new(local_enr.clone(), config).unwrap();

        // Setup RLPx session
        setup_rlp_session();

        // Example ZKP message
        let zkp_message = b"Sample ZKP message";

          // Replace with  actual list of connected helios nodes
          let node_id_1 = /* Actual Node ID for Helios 1 */;
          let enr_1 = /* Actual ENR for Helios 1 */;
          let ip_1 = "192.168.1.101".parse::<Ipv4Addr>().unwrap();
          let port_1 = 30303;
  
          let node_id_2 = /* Actual Node ID for Helios 2 */;
          let enr_2 = /* Actual ENR for Helios 2 */;
          let ip_2 = "192.168.1.102".parse::<Ipv4Addr>().unwrap();
          let port_2 = 30304;
  
          // Replace these placeholders with actual NodeRecord details
          let node_record_1 = NodeRecord::new(node_id_1, enr_1, ip_1, port_1);
          let node_record_2 = NodeRecord::new(node_id_2, enr_2, ip_2, port_2);
  
          // Add more NodeRecords as needed
        let connected_nodes: Vec<NodeRecord> = vec![
            // NodeRecord 1
            NodeRecord::new(
                // Replace with the actual NodeRecord details
                // Example: Node ID, ENR, IP, Port, etc.
            ),
            // NodeRecord 2
            NodeRecord::new(
                // Replace with the actual NodeRecord details
                // Example: Node ID, ENR, IP, Port, etc.
            ),
            // Add more NodeRecords as needed
        ];

        for node in &connected_nodes {
            match send_findnode_request(&node_discovery, node, zkp_message).await {
                Ok(_) => {
                    println!(
                        "FINDNODE Request sent successfully to node: {:?}",
                        node.udp_socket().unwrap().local_addr().unwrap()
                    );
                }
                Err(err) => eprintln!(
                    "Failed to send FINDNODE Request to node {:?}: {:?}",
                    node.udp_socket().unwrap().local_addr().unwrap(),
                    err
                ),
            }
        }
    });
}