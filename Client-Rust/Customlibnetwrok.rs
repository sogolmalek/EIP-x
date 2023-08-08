enum Message {
    RequestBalance(Address),
    ResponseBalance(Address, U256),
}
