enum Message {
    RequestBalance(Address),
    ResponseBalance(Address, U256),
    // Add more message types as needed
}
