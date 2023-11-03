use async_std::task;
use async_std::stream::StreamExt;
use async_std::sync::{Arc, Mutex};
use discv5::{Discv5, Discv5Config, Discv5Event, NodeQueryConfig, QueryId};
use discv5::enr::{CombinedKey, Enr, NodeId};
use std::net::{SocketAddr, Ipv4Addr};
use std::time::Duration;
use rand::Rng;
use tokio::time::interval;
use tokio::sync::mpsc;

// Improved ZKP structure
#[derive(Debug, Clone, PartialEq, Eq, Encodable, Decodable)]
struct ZeroKnowledgeProof {
    // Add fields for your specific ZKP message structure
    data: Vec<u8>,
}

// Enum representing different message types
#[derive(Debug, Clone, PartialEq, Eq, Encodable, Decodable)]
enum MessageType {
    FindNodeRequest(Vec<u8>, Option<ZeroKnowledgeProof>),
    ZeroKnowledgeMessage(ZeroKnowledgeProof),
    // Add other message types as needed
}

impl MessageType {
    fn findnode_request(request_id: QueryId, distances: Vec<u64>, zkp_message: Option<ZeroKnowledgeProof>) -> Self {
        MessageType::FindNodeRequest(rlp::encode(&request_id), zkp_message)
    }
}

async fn send_findnode_request(
    discv5: &Arc<Mutex<Discv5>>,
    node_id: NodeId,
    distances: Vec<u64>,
    zkp_message: Option<ZeroKnowledgeProof>,
) {
    let request_id: QueryId = rand::random();
    let request_data = MessageType::findnode_request(request_id, distances.clone(), zkp_message);

    // Use a custom configuration for this query
    let config = NodeQueryConfig {
        query_id: Some(request_id),
        ..Default::default()
    };

    discv5.lock().await.find_node(node_id, distances, config, Some(request_data)).await;
}

async fn handle_discv5_events(discv5: Arc<Mutex<Discv5>>, mut rx: mpsc::Receiver<MessageType>) {
    while let Some(event) = discv5.lock().await.next().await {
        match event {
            Discv5Event::FindNodeResult { query_id, closer_nodes, .. } => {
                println!(
                    "Received FINDNODE result for query {:?}, closer nodes: {:?}",
                    query_id, closer_nodes
                );
            }
            _ => {
                // Handle other discv5 events as needed
            }
        }
    }
}

async fn handle_zkp_messages(discv5: Arc<Mutex<Discv5>>, mut rx: mpsc::Receiver<MessageType>) {
    while let Some(message) = rx.recv().await {
        match message {
            MessageType::ZeroKnowledgeMessage(zkp) => {
                // Handle ZKP messages, e.g., verify and process
                println!("Received Zero Knowledge Proof message: {:?}", zkp);
            }
            _ => {
                // Handle other message types as needed
            }
        }
    }
}

fn main() {
    task::block_on(async {
        let local_key = CombinedKey::generate_secp256k1();
        let local_enr = local_key.generate_enr().unwrap();
        let local_node_id = local_enr.node_id();

        let config = Discv5Config {
            local_key,
            listen_address: "0.0.0.0:9000".parse().unwrap(),
            ..Default::default()
        };

        let discv5 = Arc::new(Mutex::new(Discv5::new(local_enr.clone(), config).unwrap()));

        let (tx, rx) = mpsc::channel::<MessageType>(10);

        // Spawn asynchronous tasks to handle events and messages
        let discv5_clone = Arc::clone(&discv5);
        let events_task = task::spawn(handle_discv5_events(discv5_clone, rx.clone()));

        let discv5_clone = Arc::clone(&discv5);
        let messages_task = task::spawn(handle_zkp_messages(discv5_clone, rx.clone()));

        // Example ZKP message
        let zkp_message = ZeroKnowledgeProof { data: b"Sample ZKP message".to_vec() };

        // Replace with actual connected nodes
        let node_id_1 = NodeId::random();
        let node_id_2 = NodeId::random();

        // Append ZKP messages to FINDNODE requests asynchronously
        let discv5_clone = Arc::clone(&discv5);
        let task1 = task::spawn(send_findnode_request(&discv5_clone, node_id_1, vec![0, 1, 2], Some(zkp_message.clone())));

        let discv5_clone = Arc::clone(&discv5);
        let task2 = task::spawn(send_findnode_request(&discv5_clone, node_id_2, vec![0, 1, 2], Some(zkp_message.clone())));

        // Use Tokio's interval to periodically send ZKP messages
        let mut interval = interval(Duration::from_secs(5));
        let tx_clone = tx.clone();
        let zkp_task = task::spawn(async move {
            loop {
                interval.tick().await;
                tx_clone.send(MessageType::ZeroKnowledgeMessage(zkp_message.clone())).await.unwrap();
            }
        });

        // Await completion of all tasks
        task::try_join!(task1, task2, zkp_task).unwrap();

        // Await completion of Discv5 event handling
        events_task.await.unwrap();

        // Await completion of message handling
        messages_task.await.unwrap();
    });
}