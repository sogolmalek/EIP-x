##DRAFT CODE!

This Rust code is designed to interact with Ethereum nodes using the Ethereum Node Discovery Protocol (Discv5). It aims to discover Ethereum nodes, send FINDNODE requests, and includes the ability to append Zero-Knowledge Proof (ZKP) messages to these requests.

Features
Node Discovery: The code establishes connections with Ethereum nodes and sends FINDNODE requests to discover peers.

ZKP Integration: It allows you to include ZKP messages in FINDNODE requests. ZKP messages can be retrieved from Helios nodes.

Configuration: The code is configurable with options such as local ENR setup, bootnodes, and request parameters.
