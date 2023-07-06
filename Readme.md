Xtreamly: A Stateless Account Abstraction Model that Scales Ethereum
Draft Version 2.0-June  2023

//Abstract

Enhancing Stateless Account Abstraction in Ethereum: Introducing Xtreamly, the revolutionary proposal for stateless account abstraction 
without altering the consensus-layer protocol.

Stateless AA, seperates state from validation and execution logic. In such a scenario, users may not need to provide explicit
state information to participate in an alternative  mempool. Instead, their transactions could be verified based on other criteria, 
such as cryptographic proofs and witness. To handle the state for transactions in the mempool without relying on explicit 
state information, a lightweight proxy smart contract can be integrated with an ASVC (aggregatable subvector commitment) 
scheme for more efficient, succinct and  cheaper.. 
It receives the user's transaction, along with any required cryptographic proofs or witnesses, and submits it to the network 
for validation and execution. Instead of storing the entire state, It receives the user's transaction, along with any required 
cryptographic proofs or witnesses, and submits it to the network for validation and execution.The proxy smart contract itself 
does not contain the complete state information but only holds the necessary data to facilitate the transaction. Once the transaction
is validated by the network, the proxy smart contract can update its internal state or interact with other contracts accordingly.

By combining cryptographic proofs, witnesses, and the lightweight proxy smart contract, the system can verify transactions in the
mempool without relying on explicit state information from users, improving efficiency and scalability. Once the transaction
is validated by the network, the proxy smart contract can update its internal state or interact with other contracts accordingly.

By using a lightweight proxy smart contract in this manner, the burden of maintaining and providing explicit state information 
is shifted away from the user, making participation in the mempool more accessible. It also allows for a more efficient
and scalable validation process, as the network can focus on verifying the transaction based on the provided criteria 
rather than processing the entire state for each transaction.




