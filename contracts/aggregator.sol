// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Interface for the Aggregator contract in the stateless light client
interface IAggregator {
    // Function to validate the signature of a single UserOperation
    function validateUserOpSignature(UserOperation calldata userOp) external view returns (bytes memory sigForUserOp);

    // Function to aggregate signatures of multiple UserOperations
    function aggregateSignatures(UserOperation[] calldata userOps) external view returns (bytes memory aggregatedSignature);

    // Function to validate the aggregated signature for multiple UserOperations
    function validateSignatures(UserOperation[] calldata userOps, bytes calldata signature) external view;
}

// Struct representing a block header with an aggregated subvector commitment
struct HeaderWithAggregatedSubvector {
    bytes32 parentHash;
    bytes32 uncleHash;
    address coinbase;
    bytes32 root;
    bytes32 aggregatedSubvector; // Aggregated subvector commitment field
    bytes32 txHash;
    bytes32 receiptHash;
    bytes32 bloom;
    uint256 difficulty;
    uint256 number;
    uint256 gasLimit;
    uint256 gasUsed;
    uint256 timestamp;
    bytes extra;
    bytes32 mixHash;
    uint256 nonce;
}
