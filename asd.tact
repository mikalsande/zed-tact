import "@stdlib/deploy";
import "@stdlib/ownable";

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
    owner: Address;
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

message Mint {
    amount: Int;
    receiver: Address;
}

contract TonWise with Deployable, Jetton {
    // Total supply of Tokens
    totalSupply: Int as coins;
    // total balance in contract in TON
    totalTonBalance: Int as coins;
    // Owner
    owner: Address;

    init(owner: Address) {
        self.owner = owner;
        self.totalSupply = 0;
        self.totalTonBalance = 0;
    }

    // Exchange TON for tokens.
    receive(msg: Deposit) {
        let ctx: Context = context();
        let winit: StateInit = self.getJettonWalletInit(msg.ownerReceiver);
        let tokenAddress = contractAddress(winit);
        let amountTokens = self.exchangeTonForToken(ctx.value);
        self.totalSupply += amountTokens;
        self.totalTonBalance += ctx.value;

        send(SendParameters {
            to: tokenAddress,
            value: 0,
            bounce: true,
            mode: SendRemainingValue,
            body: TokenTransferInternal {
                queryId: msg.queryId,
                amount: amountTokens,
                from: myAddress(),
                forwardTonAmount: 0,
                forwardPayload: beginCell().endCell().asSlice(),
            }.toCell(),
        });
    }

    // Exchange Tokens for TON
    fun exchangeTokenForTon(amountTokens: Int): Int {
        require(self.totalSupply >= amountTokens, "Invalid amount, not enough tokens");
        let amountTon = (amountTokens * self.totalTonBalance) / self.totalSupply;
        return amountTon;
    }

    // Exchange TON for Tokens
    fun exchangeTonForToken(amountTon: Int): Int {
        if self.totalSupply == 0 || self.totalTonBalance == 0 { return amountTon }
        let amountTokens = (amountTon * self.totalSupply) / self.totalTonBalance;
        return amountTokens;
    }

    get fun tonWiseData(): TonWiseData {
        return TonWiseData {
            totalSupply: self.totalSupply,
            totalTonBalance: self.totalTonBalance,
            owner: self.owner,
        };
    }
}

@interface("org.ton.jetton.master")
trait Jetton with Ownable {
    totalSupply: Int as coins;
    owner: Address;

    receive(msg: TokenBurnNotification) {
        self.requireWallet(msg.sender); // Check wallet
        self.totalSupply = (self.totalSupply - msg.amount); // Update supply
        if (msg.response_destination != null) {
            // Cashback
            send(SendParameters {
                to: msg.response_destination!!,
                value: 0,
                bounce: false,
                mode: SendRemainingValue,
                body: TokenExcesses { queryId: msg.queryId }.toCell(),
            });
        }
    }

    receive (msg: CreateWallet) {
        let winit: StateInit = self.getJettonWalletInit(msg.owner); // Create message
        send(SendParameters {
            to: contractAddress(winit),
            value: 0,
            bounce: false,
            mode: SendRemainingValue,
            // body: TODO: Can this be left empty?
            code: winit.code,
            data: winit.data,
        });
    }

    get fun getWalletAddress(owner: Address): Address {
        return contractAddress(self.getJettonWalletInit(owner));
    }

    fun requireWallet(owner: Address) {
        let ctx: Context = context();
        let winit: StateInit = self.getJettonWalletInit(owner);
        require(contractAddress(winit) == ctx.sender, "Invalid sender");
    }

    virtual fun getJettonWalletInit(address: Address): StateInit {
        return initOf JettonDefaultWallet(address, myAddress());
    }
}

@interface("org.ton.jetton.wallet")
contract JettonDefaultWallet {
    const minTonsForStorage: Int = ton("0.01");
    const gasConsumption: Int = ton("0.01");

    balance: Int as coins = 0;
    owner: Address;
    master: Address;

    init(owner: Address, master: Address) {
        self.balance = 0;
        self.owner = owner;
        self.master = master;
    }

    receive(msg: TokenBurn) {
        // This will be called to withdraw tokens.
        let ctx: Context = context();
        require(ctx.sender == self.owner, "Invalid sender"); // Check sender

        self.balance = self.balance - msg.amount; // Update balance
        require(self.balance >= 0, "Invalid balance");

        let fwdFee: Int = ctx.readForwardFee(); // Gas checks
        require(ctx.value > fwdFee + 2 * self.gasConsumption + self.minTonsForStorage, "Invalid value - Burn");

        // Burn tokens
        send(SendParameters {
            to: self.master,
            value: 0,
            mode: SendRemainingValue,
            bounce: true,
            body: TokenBurnNotification {
                queryId: msg.queryId,
                amount: msg.amount,
                sender: self.owner,
                response_destination: self.owner,
            }.toCell(),
        });
    }

    bounced(src: bounced<TokenBurnNotification>) {
        // Error in token burn.
        self.balance = self.balance + src.amount;
    }

    receive(msg: TokenTransferInternal) {
        let ctx: Context = context();
        if (ctx.sender != self.master) {
            let sinit: StateInit = initOf JettonDefaultWallet(msg.from, self.master);
            require(contractAddress(sinit) == ctx.sender, "Invalid sender!");
        }
        // Update balance
        self.balance = (self.balance + msg.amount);
        require(self.balance >= 0, "Invalid balance");
    }

    get fun walletData(): JettonWalletData {
        return JettonWalletData {
            balance: self.balance,
            owner: self.owner,
            master: self.master,
            code: initOf JettonDefaultWallet(self.owner, self.master).code,
        };
    }
}
