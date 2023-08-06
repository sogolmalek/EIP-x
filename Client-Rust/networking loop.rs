use libp2p::core::identity::Keypair;
use libp2p::tcp::TcpConfig;
use libp2p::identity::{secp256k1, Keypair};
use libp2p::noise::{Keypair, NoiseConfig, X25519Spec};
use libp2p::tcp::TcpConfig;
use libp2p::yamux::YamuxConfig;
use tokio::task;

impl LightClient {
    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let local_key = identity::Keypair::generate_secp256k1();
        let local_peer_id = local_key.public().into_peer_id();
        println!("Local peer id: {:?}", local_peer_id);

        let transport = libp2p::build_development_transport(local_key)?;
        let mut swarm = Swarm::new(self.discovery, transport, local_peer_id.clone());

        loop {
            match swarm.select_next_some().await {
                // Handle events from the swarm (new peers, incoming messages, etc.)
                _ => {}
            }
        }
    }
}
