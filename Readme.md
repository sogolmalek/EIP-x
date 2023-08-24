EIP-x, Xtreamly
Stateless Account abstraction that scales Ethereum.

## Motivation
Ethereum’s state grows rapidly, burdening consensus nodes with 35 GB state and 100 GB with proofs, increasing every six months. The increasing state size hampers scalability, imposing storage and processing burdens on nodes. Participants pay one-time costs perpetually, raising economic concerns. Additionally, ERC-4337 wallets incur elevated gas costs (~42000 gas for basic operations) due to multiple storage read/write costs, business logic overhead, and log payments not required in EOAs. A stateless-based solution is essential to maintain Ethereum’s efficiency and long-term viability. Additionally, the light clients, which do not store the entire state but rely on simplified verification mechanisms, struggle to efficiently access and validate the state data against the mainnet. The lack of a concise and constant-sized proof of the current state limits light clients’ ability to interact with the blockchain seamlessly. Therefore we Propose a stateless Account abstraction model which doesn’t force any change in the current Ethereum core but can solve the above problems at the account level. The advantage of having succinct zk proof of the state is that it is possible to broadcast it across the nodes.

## ​​Project description

The challenge at hand is to enable stateless verification of blocks in the blockchain system. Currently, to validate a block, light clients must request the state piece by piece, and this results in many light clients frequently burdening the full node altogether. The goal is to allow clients to verify the correctness of individual blocks without needing a full state. Instead, clients can rely on compact witness files generated by state-holding nodes, which contain the portion of the state accessed by the block along with proof of correctness. This stateless verification offers several benefits, including supporting sharding setups and reducing reliance on validators.
# Issue 1: introducing new Entity for Stateless AA, trustless State provider #1

this entity consists of Ethereum light clients providing the latest state, peer-to-peer network and ZK circuit. We introduce a decentralized peer-to-peer network with a state provider entity, collaborating to validate and generate ZK-SNARKs proofs for state information.A lightweight client verifies proofs and validates state data from the state provider.

The state provider entity is at the core of stateless AA, and leverages the Helios light client to query the latest block state in a fully trustless manner. By implying
a peer-to-peer network, we can ensure the latest state of the block in a fully trustless and decentralized manner. Then we are able to provide succinct and constant-sized proof of the current state and propagate the ZK-proof of witness needed for execution. The current zk-proof of the state can then be propagated to all light clients at once. Light clients can effortlessly verify this proof against the mainnet, improving scalability, and reducing resource requirements for light clients. Users and applications can efficiently interact with the blockchain, relying on the security and integrity of zk-proofs to verify the state. This approach enables more participants to engage with the network without needing the computational resources and storage capacity required for full nodes.

# Issue 2: introducing new Entity for Stateless AA, The stateless verifier:

The stateless verifier acts as a light client that receives a zero-knowledge proof (zk proof) of a block state and the new state after a transaction is executed. This verifier will provide verification of the transaction and include the new state on the chain. The verifier receives a new block containing the transaction, the zk proof of the witness including the latest state (before execution), and the resulting new state after the transaction. The verifier extracts the transaction and the zk proof from the received block. It can then verify the correctness of the transaction execution without needing to access the entire state.

If the transaction is valid, the verifier applies the transaction to the state it currently holds, thus deriving the new state.

## Roadmap


Roadmap for MVP Development of Hybrid Witness Sharing and Stateless Account Abstraction Proposal:

Phase 1: Research and Planning

1. Conduct In-Depth Research: Perform a comprehensive study of existing stateless account abstraction models,
   hybrid witness sharing techniques, and their potential integration with Ethereum's current ERC-4337 architecture.
   Identify any modifications required to adapt the ERC-4337 flow for the proposed solution.

2. Define Use Cases: Identify and document specific use cases that will be showcased in the MVP to demonstrate
   the benefits of the proposal. Consider scenarios where witness sharing and stateless account abstraction can lead
   to improved efficiency, reduced gas consumption, and enhanced scalability.

3. MVP Scope and Features: Define the scope of the MVP, including the core features that will be implemented,
    such as state provider with zk-proof, witness sharing model, and the necessary modifications to the ERC-4337 architecture.

Phase 2: Development

1. Implement Witness Sharing Model: Develop the witness sharing model, leveraging cryptographic techniques like Merkle trees or zk-SNARKs.
   The model should efficiently share witness information among multiple transactions, optimizing gas consumption for transaction validation.

2. Stateless Account Provider with zk-Proof: Create a state provider that stores essential data required for transaction execution,
  adhering to the stateless account abstraction concept. Implement zero-knowledge proofs (zk-proofs) to ensure data integrity and security.

3. Implement Statelss Verifier as a light client

Phase 3: Integration and Testing

1.  Modify ERC-4337 Architecture: Make minor modifications to the ERC-4337 architecture to accommodate the proposed hybrid witness
   sharing and stateless account abstraction. Ensure compatibility with existing smart contracts and functionalities while integrating the new components.
3.  Integration with Bundler and Entry Point Smart Contract: Integrate the developed state provider and witness sharing model
   with the bundler and entry point smart contract of ERC-4337. Verify that all components work seamlessly together to
   facilitate transaction execution and witness sharing.

4. MVP Testing: Conduct extensive testing to ensure the MVP's functionality, security, and performance. Perform unit testing,
   integration testing, and simulate real-world scenarios to validate the proposal's benefits and use cases.


Continuous Improvement: Continuously refine and optimize the MVP based on community feedback and new research findings.
 Explore potential enhancements, security updates, and additional features that can be incorporated to further improve the proposal's efficiency and scalability.

By following this roadmap, the development team can create a functional MVP showcasing the benefits of the hybrid witness sharing and stateless account abstraction proposal without the dependency on Verkle trees or protocol-level changes. The MVP can serve as a powerful demonstration of the proposed solution's potential and open up new possibilities for the Ethereum community.


## Goal of the project

Success for the  Stateless Account Abstraction project would be achieved when the proposed solution is 
fully developed, implemented, and widely adopted within the Ethereum community. The end goal is to demonstrate a functional MVP 
that showcases the benefits of hybrid witness sharing and stateless account abstraction while providing tangible improvements to 
Ethereum's efficiency, scalability, and usability. All are accomplished without dependency to a statelessness roadmap or conduction 
of changes within the core. 


## Collaborators
Sogol Malek
Luke Schoen
Elvis Sabanovic
### Fellows 
Luke Schoen (EPF)

### Mentors

Guillaume ballet 
Matt Garnett
Yoav Weiss
Sina Mahmoodi 

## Resources
https://github.com/sogolmalek/EIP-x

https://docs.google.com/presentation/d/1heCbSH1Mj1oG0aPamk0yQ5Mo9sLJfAmt71lzBe-0-C4/edit?usp=drive_link
