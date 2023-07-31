// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SignatureApproval {
    address public owner;
    mapping(address => bool) public approvedSigners;

    modifier onlyOwner() {
        require(msg.sender == owner, "Only the contract owner can call this function.");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    // The contract owner can add signers who can approve transactions
    function addSigner(address _signer) external onlyOwner {
        approvedSigners[_signer] = true;
    }

    // The contract owner can remove signers
    function removeSigner(address _signer) external onlyOwner {
        approvedSigners[_signer] = false;
    }

    // Check if a signature is valid
    function isValidSignature(
        bytes32 _message,
        uint8 _v,
        bytes32 _r,
        bytes32 _s
    ) internal view returns (bool) {
        return ecrecover(_message, _v, _r, _s) == owner || approvedSigners[ecrecover(_message, _v, _r, _s)];
    }

    // Function to perform some action that requires signature approval
    function performAction(bytes32 _message, uint8 _v, bytes32 _r, bytes32 _s) external {
        require(isValidSignature(_message, _v, _r, _s), "Invalid signature.");
        // Adding the logic here  to perform when the signature is valid.
        // One can update state variables or execute some logic.
    }
}
