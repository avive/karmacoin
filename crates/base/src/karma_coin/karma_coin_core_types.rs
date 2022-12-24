//////////////////
// Basic KarmaCoin data types
/////////////////

/// Derived from a public key
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AccountId {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// A non-negative coin amount
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Amount {
    #[prost(uint64, tag = "1")]
    pub value: u64,
    #[prost(enumeration = "CoinType", tag = "2")]
    pub coin_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(message, optional, tag = "1")]
    pub free: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "2")]
    pub reserved: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "3")]
    pub misc_frozen: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "4")]
    pub fee_frozen: ::core::option::Option<Amount>,
}
/// An public encryption key
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PreKey {
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<PublicKey>,
    #[prost(uint32, tag = "2")]
    pub id: u32,
    #[prost(enumeration = "KeyScheme", tag = "3")]
    pub scheme: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyPair {
    #[prost(message, optional, tag = "1")]
    pub private_key: ::core::option::Option<PrivateKey>,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<PublicKey>,
    #[prost(enumeration = "KeyScheme", tag = "3")]
    pub scheme: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(enumeration = "KeyScheme", tag = "1")]
    pub scheme: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MobileNumber {
    /// always up to 12 digits which including country code
    #[prost(string, tag = "1")]
    pub number: ::prost::alloc::string::String,
}
/// user on-chain data
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// account id derived from a public key
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<AccountId>,
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
    /// unique across the system
    #[prost(string, tag = "3")]
    pub user_name: ::prost::alloc::string::String,
    /// verified current number
    #[prost(message, optional, tag = "4")]
    pub mobile_number: ::core::option::Option<MobileNumber>,
    #[prost(message, repeated, tag = "5")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
    #[prost(message, repeated, tag = "6")]
    pub trait_scores: ::prost::alloc::vec::Vec<TraitScore>,
    /// one-time enc pre-keys for e2e messaging
    #[prost(message, repeated, tag = "7")]
    pub pre_keys: ::prost::alloc::vec::Vec<PreKey>,
}
/// Phone verifier is an entity that verifies account mobile phone numbers
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoneVerifier {
    /// verifier account id
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<AccountId>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
// Data that is stored on chain

//
//message OnChainData {
//repeated User users = 1;
//repeated PhoneVerifier sms_verifiers = 2;
//repeated TraitName traits = 3; // char trait ids supported by the system
//repeated SignedTransaction transactions = 4; // signed transactions- all for archive, only recent for standard nodes
//repeated Block blocks = 5; // the blockchain
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(uint64, tag = "1")]
    pub time: u64,
    #[prost(message, optional, tag = "2")]
    pub author: ::core::option::Option<AccountId>,
    #[prost(uint64, tag = "3")]
    pub height: u64,
    /// of the signed transactions in this block
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub transactions_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// total fees paid in this block
    #[prost(message, optional, tag = "5")]
    pub fees: ::core::option::Option<Amount>,
    /// digest of block in consensus at the previous height
    #[prost(bytes = "vec", tag = "6")]
    pub prev_block_digest: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "7")]
    pub signature: ::core::option::Option<Signature>,
    /// block digest includes hash of all above data
    #[prost(bytes = "vec", tag = "8")]
    pub digest: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TraitName {
    #[prost(enumeration = "CharTrait", tag = "1")]
    pub char_trait: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// A set of traits
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Traits {
    #[prost(message, repeated, tag = "1")]
    pub named_traits: ::prost::alloc::vec::Vec<TraitName>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TraitScore {
    #[prost(enumeration = "CharTrait", tag = "1")]
    pub r#trait: i32,
    #[prost(uint32, tag = "2")]
    pub score: u32,
}
/// Update user info
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserV1 {
    /// new requested nickname
    #[prost(string, tag = "1")]
    pub nickname: ::prost::alloc::string::String,
    /// Updated verified number
    #[prost(message, optional, tag = "2")]
    pub mobile_number: ::core::option::Option<MobileNumber>,
    /// verifier attestation regarding the number and the account
    #[prost(message, optional, tag = "3")]
    pub verify_number_response: ::core::option::Option<VerifyNumberResponse>,
}
/// Basic payment transaction with optional character appreciation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentTransactionV1 {
    /// dest is always a mobile number (of a user or a non-user)
    #[prost(message, optional, tag = "1")]
    pub to: ::core::option::Option<MobileNumber>,
    /// amount in tokens to transfer
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<Amount>,
    /// char trait set by sender. e.g. smart
    #[prost(enumeration = "CharTrait", tag = "3")]
    pub char_trait: i32,
}
/// Created and signed by a verifier
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNumberResponse {
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    #[prost(enumeration = "VerifyNumberResult", tag = "2")]
    pub result: i32,
    #[prost(message, optional, tag = "3")]
    pub account_id: ::core::option::Option<AccountId>,
    #[prost(message, optional, tag = "4")]
    pub mobile_number: ::core::option::Option<MobileNumber>,
    #[prost(string, tag = "5")]
    pub nickname: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub signature: ::core::option::Option<Signature>,
}
/// new user transactions submitted by users
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewUserTransactionV1 {
    /// initial user balance
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
    /// Evidence from a valid verifier about the new user
    #[prost(message, optional, tag = "2")]
    pub verify_number_response: ::core::option::Option<VerifyNumberResponse>,
}
/// serialized transaction data
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TransactionData {
    /// binary transaction data (e.g. NewUserTxV1, PaymentV1, etc...)
    #[prost(bytes = "vec", tag = "1")]
    pub transaction_data: ::prost::alloc::vec::Vec<u8>,
    /// transaction type for deserialization
    #[prost(enumeration = "TransactionType", tag = "2")]
    pub transaction_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SignedTransaction {
    /// account this tx is signed by
    #[prost(message, optional, tag = "1")]
    pub signer: ::core::option::Option<AccountId>,
    /// time transaction was signed
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    /// tx nonce
    #[prost(uint64, tag = "3")]
    pub nonce: u64,
    /// network fee provided by sender
    #[prost(message, optional, tag = "4")]
    pub fee: ::core::option::Option<Amount>,
    /// binary transaction data
    #[prost(message, optional, tag = "5")]
    pub transaction_data: ::core::option::Option<TransactionData>,
    /// network id to avoid confusion with testnets
    #[prost(uint32, tag = "6")]
    pub network_id: u32,
    /// signer signature on all of the above data
    #[prost(message, optional, tag = "7")]
    pub signature: ::core::option::Option<Signature>,
}
/// Pending transactions are transactions that are not yet on chain
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MemPool {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<SignedTransaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedTransactionWithStatus {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<SignedTransaction>,
    /// transaction status
    #[prost(enumeration = "TransactionStatus", tag = "2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewUserEvent {
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    #[prost(message, optional, tag = "2")]
    pub signer: ::core::option::Option<AccountId>,
    #[prost(enumeration = "SignupMethod", tag = "3")]
    pub sign_up_method: i32,
    /// for invites - inviter gets a reward - protocol constant
    #[prost(message, optional, tag = "4")]
    pub referred_reward: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "5")]
    pub signup_reward: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "6")]
    pub verifier: ::core::option::Option<PhoneVerifier>,
}
/// Transaction added to ledger
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TransactionEvent {
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    /// ledger height of execution
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(message, optional, tag = "3")]
    pub transaction: ::core::option::Option<SignedTransaction>,
    #[prost(bytes = "vec", tag = "4")]
    pub transaction_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "ExecutionResult", tag = "5")]
    pub result: i32,
    #[prost(string, tag = "6")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(enumeration = "FeeType", tag = "7")]
    pub fee_type: i32,
}
/// A collection of events for a transaction
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TransactionEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<TransactionEvent>,
}
/// Blockchain aggregated data
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct BlockchainStats {
    /// last block production time
    #[prost(uint64, tag = "1")]
    pub last_block_time: u64,
    /// current block height
    #[prost(uint64, tag = "2")]
    pub tip_height: u64,
    /// total number of executed transactions
    #[prost(uint64, tag = "3")]
    pub transactions_count: u64,
    /// total number of payment transactions
    #[prost(uint64, tag = "4")]
    pub payments_transactions_count: u64,
    /// total number of verified user accounts
    #[prost(uint64, tag = "5")]
    pub users_count: u64,
    /// total tx fees collected by block producers
    #[prost(uint64, tag = "6")]
    pub fees_amount: u64,
    /// total number of kCents minted by the protocol since genesis
    #[prost(uint64, tag = "7")]
    pub minted_amount: u64,
    /// total number of kCents in circulation by minting. Not including pre-mint
    #[prost(uint64, tag = "8")]
    pub circulation: u64,
    /// total tx fee subsidies issued by the protocol
    #[prost(uint64, tag = "9")]
    pub fee_subs_count: u64,
    #[prost(uint64, tag = "10")]
    pub fee_subs_amount: u64,
    #[prost(uint64, tag = "11")]
    pub signup_rewards_count: u64,
    #[prost(uint64, tag = "12")]
    pub signup_rewards_amount: u64,
    #[prost(uint64, tag = "13")]
    pub referral_rewards_count: u64,
    #[prost(uint64, tag = "14")]
    pub referral_rewards_amount: u64,
    #[prost(uint64, tag = "15")]
    pub validator_rewards_count: u64,
    #[prost(uint64, tag = "16")]
    pub validator_rewards_amount: u64,
}
/// Block events
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockEvent {
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "4")]
    pub transactions_events: ::prost::alloc::vec::Vec<TransactionEvent>,
    #[prost(uint64, tag = "5")]
    pub total_signups: u64,
    #[prost(uint64, tag = "6")]
    pub total_payments: u64,
    #[prost(message, optional, tag = "7")]
    pub total_fees: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "8")]
    pub total_signup_rewards: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "9")]
    pub total_referral_rewards: ::core::option::Option<Amount>,
}
/// Supported built-in coin types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CoinType {
    /// $KCents
    Core = 0,
    /// $KCStableCents
    Stable = 1,
}
/// Supported signature schemes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyScheme {
    Ed25519 = 0,
}
/// Supported char traits. Enum values are the traits unique id
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CharTrait {
    Kind = 0,
    Helpful = 1,
    Smart = 2,
}
/// transactions

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionType {
    PaymentV1 = 0,
    NewUserV1 = 1,
    UpdateUserV1 = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VerifyNumberResult {
    NicknameTaken = 0,
    InvalidCode = 1,
    InvalidSignature = 2,
    /// number is registered to another account
    NumberAlreadyRegisteredOtherAccount = 3,
    //// an account with this number already exists
    NumberAlreadyRegisteredThisAccount = 4,
    Verified = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionStatus {
    Unknown = 0,
    Pending = 1,
    Rejected = 2,
    OnChain = 3,
}
/// events - emitted by runtime, all stored by archive nodes only
/// full nodes have only recent events
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignupMethod {
    /// user was referred by another user
    SignUpMethodReferred = 0,
    /// user wasn't referred - signed up
    SignUpMethodSignup = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FeeType {
    /// fee provided by the protocol
    Mint = 0,
    /// fee provided by the transaction signer
    User = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionResult {
    Executed = 0,
    Invalid = 1,
}
