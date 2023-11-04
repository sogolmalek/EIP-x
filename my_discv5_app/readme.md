##DRAFT CODE!

This Rust code is designed based on Discv5 communication protocol to interact with Ethereum nodes using the Ethereum Node Discovery Protocol (Discv5).This protocol aims to be integrated with Trin licght client and allows peers to exchange zkp of recent state, allowing peers to verify account and storage data without having to download intermediate Merkle trie nodes. It aims to discover Ethereum nodes, send FINDNODE requests, and includes the ability to append Zero-Knowledge Proof (ZKP) messages to these requests. is a peer-to-peer network using the `discv5` crate for Ethereum Node Discovery.  We have defined a  `MessageType` enum for different message types, including `FindNodeRequest` and `ZkpMessage` (Zero-Knowledge Proof message)  The `send_findnode_request` function generates a FindNode request with a random request ID and specific distances. It then combines this request with a ZKP message and sends it to a specified node using RLPx communication. The `send_message` function encodes a message using RLP and simulates RLPx communication by sending the encoded message.
For each connected node, a FindNode request with the ZKP message is sent asynchronously.

we have  implemented a loop that periodically sends FINDNODE requests to the connected nodes.adjusted the sleep duration to 10 milliseconds, by using the tokio library, we can  have  more fine-grained control over asynchronous tasks and timers.




Features
Node Discovery: The code establishes connections with Ethereum nodes and sends FINDNODE requests to discover peers.

ZKP Integration: It allows you to include ZKP messages in FINDNODE requests. ZKP messages can be retrieved from Helios nodes.

Configuration: The code is configurable with options such as local ENR setup, bootnodes, and request parameters.
30-oct-2023: 
implemented a loop that periodically sends FINDNODE requests to the connected nodes.adjusted the sleep duration to 10 milliseconds, you can use the tokio library, which provides more fine-grained control over asynchronous tasks and timers

