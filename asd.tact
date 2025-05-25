message Deposit {
    queryId: Int as uint64;
    ownerReceiver: Address;
}

message CreateWallet {
    owner: Address;
}

message(0x178d4519) TokenTransferInternal {
    queryId: Int as uint64;
    amount: Int as coins;
    from: Address;
    forwardTonAmount: Int as coins;
    forwardPayload: Slice as remaining;
}


message(0x7bdd97de) TokenBurnNotification {
    queryId: Int as uint64;
    amount: Int as coins;
    sender: Address;
    response_destination: Address?;
}

struct JettonWalletData {
    balance: Int;
    owner: Address;
    master: Address;
    code: Cell;
}

struct TonWiseData {
    totalSupply: Int as coins;
    totalTonBalance: Int as coins;
    owner: Address
}

message(0x595f07bc) TokenBurn {
    queryId: Int as uint64;
    amount: Int as coins;
    response_destination: Address?;
    custom_payload: Cell?;
}

message(0xd53276db) TokenExcesses {
    queryId: Int as uint64;
}
