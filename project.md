# EIP-X Proposal

Stateless Light Client That Consumes ZKP With Efficient Access To Specific Segments Of The State.

Ethereum Improvement Proposal (EIP) Title: Stateless Light Client with Zero-Knowledge Proof (ZKP) Integration for Efficient State Access

**Abstract:**
In the realm of blockchain technology, advancing the capabilities of light clients is crucial for enhancing the scalability and adoption of the Ethereum network. This proposal, EIP-X, introduces a stateless light client framework empowered with zero-knowledge proofs (ZKPs), facilitating efficient access to specific segments of the state. By integrating ZKPs into lightweight client operations, this proposal aims to mitigate the limitations of traditional light clients, particularly in the context of staking activities and Flashbots' transactions.

**Motivation:**
Traditional light clients impose burdens on full nodes by incrementally requesting block states, hindering network scalability. While Verkle trees have improved lightweight client transitions between blocks, challenges persist in ensuring accurate state root confirmation. Additionally, the inability of light clients to handle ZKPs adds complexity to the ecosystem. EIP-X addresses these concerns by introducing a stateless light client architecture equipped with ZKP integration.

**Specification:**
1. **Witness Generation:** A witness generator, based on Geth, subscribes to blockchain events and extracts cryptographic witnesses upon block generation. These witnesses form the basis for generating ZKPs.

2. **ZKP Generation:** A ZK proof of the witness is generated using the ZK Execution Environment (ZKEVM) module.

3. **Dissemination:** The ZKPs are disseminated to participating lightweight client nodes via the Ethereum standard communication protocol Discv5.

4. **Efficient State Access:** Lightweight clients can efficiently incorporate ZKPs into their operations, enhancing accessibility to specific segments of the state.

5. **Use Cases:**
   - **Staking Efficiency:** Validators can provide a single ZKP of their staking activity, reducing computational burden and enabling efficient restaking at scale.
   - **Flashbots Mitigation:** Stateless clients capable of consuming ZKPs can mitigate risks associated with Flashbots' zero gas price transactions, enhancing network security and reliability.

**Benefits:**
- Improved scalability and adoption of the Ethereum network.
- Enhanced security and reliability through ZKP integration.
- Efficient access to specific segments of the state, facilitating diverse use cases.

**Conclusion:**
EIP-X proposes a stateless light client architecture integrated with ZKP capabilities, addressing key limitations of traditional light clients. By enhancing scalability, security, and accessibility, this proposal paves the way for broader Ethereum network adoption and innovation.

This EIP is open for discussion and feedback from the Ethereum core developer community and stakeholders.
---
## Collaborators

- Sogol Malek
- Mohammadreza Nakhle

## Mentors

- Guillaume Ballet (Lead @Verkle Trie)
- Sina Mahmoudi (Geth Team)
- Portal Team
- Daniel Marcez (protocol eng @Flashbots)


## PowerPoint Link

[Link to PowerPoint Presentation](https://docs.google.com/presentation/d/1H-ZUW5vUM5Tm30q5tEC_ZOdJg2cqSVp19bHOzN2LzFE/edit?usp=sharing)

## Github

[Github Repository Link](https://github.com/sogolmalek/EIP-x.git)
