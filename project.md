EIP-X proposal 


EIP-x 
 Stateless LC That Consumes ZKP With Efficient Access To The Specific Segments Of The State.


## Motivation
In the dynamic landscape of blockchain technology, addressing the limitations of traditional light clients is paramount for the widespread adoption and efficient functioning of the Ethereum network. Conventionally, light clients have burdened full nodes by incrementally requesting block state, posing challenges to network scalability. The integration of Verkle trees has provided a significant step forward, facilitating smoother transitions for lightweight clients between blocks. However, a lingering concern lies in the assurance of accurate state root confirmation, highlighting the need for further advancements. The inability of light clients to handle zero-knowledge proofs compounds the complexity. 

As sequence of urgent need for super light weight client that consumes zkp, our EIp can address oem challenges that we face constantly : Front-running and sandwich attacks. 

In mev utopia model we want specializian across ither things such as max competition and no privileged actors, to get to the mev utopia world ideally we need to enable trustless collaboration and maximaize competition in mev supply chain, but its quite hard because parties naturally dont trust eachother. Validators want to propose  a block that has the most value but we dont want them to necessarily have sophisticated infrastructures so they source that out ot builders, but if validators can see inside the blocks, they can steal it from builders, or front run users. As consequence, blockbuilders send their blocks tosome  trusted validators, for example large well knowns, and solo stakers would be unable to receive competitive mev rewards, 

So the take away here : builders need privacy from validators.. 

 To that context, we were working on a cool concept in from of a light client that consuems zkp and is so light that operates in stateless manner. 




## Project Description:
Our endeavor,EIP-x, embodies a thoughtful exploration of trin light clients' capabilities as the base of light client. 
The first component, is  witness generator a fork of Geth that subscribes to the blockchain event, whenever a new block has beengenerated will automatically fetsch the  most recent block state of the block. This progress entails the extraction of a cryptographic witness, which in turn serves as the foundation for the creation of zero-knowledge proofs (ZKPs). A ZKEVM module will generate the zk proof of the witness. These ZKPs, integral to our approach, are subsequently disseminated to all participating lightweight client (LC) nodes across the peer to peer network of nodes. Therefore all nodes will have the entire last state with downloading only a succinct zkp of witness corresponded to the last block on chain. In the context of the Portal Network, Our approach centers on empowering Trin clients to efficiently incorporate ZKPs into their operations.we leverage ethereum standard communication protocol Discv5 to propagate the zkp across Trin’s p2p network at once. 


In cases where cache-related inconsistencies or failures may surface, participants can confidently resort to the ZK proofs of the latest state. These cryptographic proofs serve as a dependable means to verify the integrity of cached data fragments, reinforcing trust and reliability within the system. This project project endeavors to significantly enhance the efficiency, security, and accessibility of blockchain data for stateless light clients, with a product direction being mobile friendly light client to significantly increase the decentralization and reduce the barrier tobe verifier node with least resource consumption. 


## Goal of the project


##Important Use-case1: 
Being stateless Node for Flashbot
Problem: 
Flashbots' 0 gas price transactions, paying miners via smart contracts, risk a Denial-of-Service (DOS) vector. Miners must simulate transactions to assess profitability, making them vulnerable to spam attacks without cost. This differs from regular Ethereum transactions with inherent fees and node-based mempool filtering.
Potential Solution with EIP-X: Stateless clients, which can consume  zkps, have the potential to mitigate the problem above


## mportant Use-case 2: 

The entire idea of eip-x has stated by proposing a light client that can verify the zk message type and can propagate a zkp across p2p network at one call 

Let's start by understanding the challenges we aim to overcome. Front running, the act of exploiting advanced knowledge of transactions to gain an unfair advantage, and sandwich attacks, where malicious actors position themselves to profit from others' trades, are persistent threats in our ecosystem.


Enter zero-knowledge proofs (ZKPs), a powerful cryptographic tool that allows us to prove the correctness of computations without revealing sensitive information. Our proposal centers around integrating ZKPs into the order matching process to enhance privacy and security.

The solution begins with a commit-reveal scheme, where users commit to their trades without revealing specifics. What sets us apart is the integration of ZKPs, allowing users to prove the validity of their commitments without exposing critical details.


To mitigate the risk of front running, we introduce time-locked execution. Trades are only revealed and processed after a specified time period, reducing the window of opportunity for malicious actors.

Acknowledging that ZKP generation may take time, we implement batch processing and explore parallelization techniques to optimize the efficiency of ZKP generation, ensuring scalability and speed.


## Colaborators:
-Sogol Malek
-Mohammadreza Nakhle

## Mentors 
-Guillaume Ballet (Lead @Verkle Trie)
-Sina Mahmoudi (Geth Team)
-Portal Team 
-Daniel Marcez (protocol eng @Flashbots)
-Cc: Hasu, Strategic lead @Flashbots

## Powerpoint Link: 
https://docs.google.com/presentation/d/1s2YbWEqGq7IINof2ILLk9UOUFcuYgmH-IXrCJHhLaOw/edit?usp=drive_link

## Github:
https://github.com/sogolmalek/EIP-x.git 


