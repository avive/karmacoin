# Subnet
Subnet is a web3 digital communications platform.
This repository is an experimental implementation of the Subnet protocol v0.1 in Rust.
The project includes code for service provider's full p2p nodes, terminal clients and comperhansive scenario tests.
It also includes the [Subnet interactive playground](https://asciinema.org/a/W51QFvKxyFq64kOQJh2gVRqVL). 
Use the playground to experiment with protocol features.

## Building
```cargo build```

## Testing
```cargo test```

## Running the playground
1. Build the project.
1. Copy the playground config files from `debug_config` to `target/debug`
1. `./target/debug/playground`

---

## Dev Notes

All timestamps should be in nanosecs using chrono

```rust
use chrono::prelude::*;
let t = Utc::now().timestamp_nanos() as u64
```

#### Architecture

- Uses `xactors` (over `tokio` runtime) actors pattern. Async actors that can return responses to messages.
- Uses `tonic` and `prost` for grpc
- Xactor used as local lib - enhanced with new features that required low-level integration such as pub/sub.
- High-level system components are all `xactor services` - registered in the system and restarted automatically when called after they crash.
- Store is implemented using `rocksdb`.
- Network protocols are defined in `protobufs` language.

- `base` - shared types.
- `crypto` - low-level crypto lib
- `common` - shared functionality between a client and server that uses types and base, crypto and other libs.
- `client` - Simple client with a grpc api for code reuse in testing servers and client-to-client p2p flows.
- `client-app` - Simple terminal client app with support to config file, cli flags and logging.
- `dr` - Implementation of double-ratchet protocol.
- `server` - Server implementation.
- `server-app` - 

#### Master TODO list
- Handle out of order DR messages properly
    - count, prev-count, etc...
- Update to tokio 1.0 and xactor to the latest release (embedded code)    
- Implement async s/kad algo 
- Nodes Blockchain Design 
  - Bond amount to participate to avoid sybil attack - real cost to create a node and participate in consensus. e.g. Substrate w PoS mechanisms. 
- Properly support net-id in bundles and p2p message + init entities w net id from config file.        
- Add user update bundle feature (new prekey) with provider
- Implement the switch provider flow - user sets a new provider and stops service from an old provider.

-------

### Crypto

- Mock Crypto L1 and Crypto L2 
- CoreCoin - $UP  
- StableCoin- $SUP
- Account 
- Amounts

L1ServiceMock
- Pay(from: Account, amount: CoreCoin, to: Account, gas: 1, gasPrice: 1); /// will create to account
- PurchaseStableCoin(from: Account, amount: CoreCoinUnit, to: Account);
- GetBondAmount() -> AmountCoreUnits // get the required bond amount in core coin per provider

// Providers Bonding
- CreateBond(from: account, amount: CoreCoinUnit, provider: EntityId); // create a bond for a provider - BondI
- CheckBondStatus(provider: EntityId); // anyone can call
- RedeemBond(provider: EntityId) // redeem bond back to the address that paid for it. signed by entity id.

- LockForLayerTwoPayments(from: account, amount: StableCoin, gas: 1, gasPrice: 1); /// will create to account -> locked_funds_address 
- GetBalance(account) -> CoreCoin balances. Available and Locked (Staked).
- CreateNewAccount() -> account // create new coin account and return account address. Give each account CoreCoin and StableCoin for now.
  in the final product this will be created only when payment is made to this account from another account.
- Stable coin related treasury functions - rules baked into L1 consensus 

L2ServiceMock
- Pay(amount: Amount, coin:Stable/Core from: Account, to: Account, gas: 1, gasPrice: 1) -> Receipt;
- Balance(account) => StableCoin balance;
- MovetoL1(account: address, amount: StableCoin);
- MoveFromLayerOne(account: address, amount: StableCoin);
- VerifyReceipt(receipt) -> bool; // verify an L2 receipt

Wallet
- Holds accounts:
  - Keypairs
  - address (20 bytes of public key)
  - balances 
  - spending budget
    
- SetMonthlyL2SpendingBudget(amount) // set monthly spending budget that client can use automatically to pay for provider services.
- GetCurrentL2SpendingMonthlyBudget() 
- L2Pay(amount, to, gas, gasPrice, keypair, invoice_id) // will make payment while within monthly budget -> Receipt
- L1Pay(amount, to, gas, gasPrice, keypair, invoice_id)
- Balance(account)
- PaymentHistory(account)
- CreateAccount(key-pair) // creat an L1 account (it will have נשךשמבק)
- LockFundsForL2Payments(keypair, amount)
- Accounts() // shows managed accounts

---

Copyright (c) 2021 by the [Subnet Authors](https://github.com/subnetter?tab=repositories). This work is licensed under the Subnet License v0.1.0.




