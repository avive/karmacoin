//////////////////
//
// Basic KarmaCoin data types
//
/////////////////

/// Derived from a public key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountId {
    /// derived from pub key
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// A non-negative coin amount
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Amount {
    #[prost(uint64, tag = "1")]
    pub value: u64,
    #[prost(enumeration = "CoinType", tag = "2")]
    pub coin_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(message, optional, tag = "1")]
    pub free: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "3")]
    pub reserved: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "4")]
    pub misc_frozen: ::core::option::Option<Amount>,
    #[prost(message, optional, tag = "5")]
    pub fee_frozen: ::core::option::Option<Amount>,
}
/// An public encryption key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreKey {
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<PublicKey>,
    #[prost(uint32, tag = "2")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyPair {
    #[prost(message, optional, tag = "1")]
    pub private_key: ::core::option::Option<PrivateKey>,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<PublicKey>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(uint32, tag = "1")]
    pub scheme_id: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileNumber {
    /// 12 digits
    #[prost(string, tag = "1")]
    pub number: ::prost::alloc::string::String,
}
/// user on-chain data
#[derive(Clone, PartialEq, ::prost::Message)]
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
/// Data that is stored on chain
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnChainData {
    #[prost(message, repeated, tag = "1")]
    pub users: ::prost::alloc::vec::Vec<User>,
    #[prost(message, repeated, tag = "2")]
    pub sms_verifiers: ::prost::alloc::vec::Vec<PhoneVerifier>,
    /// char trait ids supported by the system
    #[prost(message, repeated, tag = "3")]
    pub traits: ::prost::alloc::vec::Vec<TraitName>,
    /// signed transactions- all for archive, only recent for standard nodes
    #[prost(message, repeated, tag = "4")]
    pub transactions: ::prost::alloc::vec::Vec<SignedTransaction>,
    /// the blockchain
    #[prost(message, repeated, tag = "5")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    pub author: ::core::option::Option<AccountId>,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// of the signed transactions in this block
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub transactions_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "4")]
    pub signature: ::core::option::Option<Signature>,
    /// digest of block in consensus at the previous height
    #[prost(bytes = "vec", tag = "5")]
    pub prev_block_digest: ::prost::alloc::vec::Vec<u8>,
    /// block digest includes hash of all above data
    #[prost(bytes = "vec", tag = "6")]
    pub digest: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraitName {
    #[prost(enumeration = "CharTrait", tag = "1")]
    pub r#trait: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraitScore {
    #[prost(enumeration = "CharTrait", tag = "1")]
    pub r#trait: i32,
    #[prost(uint32, tag = "2")]
    pub score: u32,
}
/// Update user info
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserV1 {
    /// Only user may change his account id by signing with private key of old accountId
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<AccountId>,
    /// Only user may change his own nickname. Nicknames are unique.
    #[prost(string, tag = "2")]
    pub nickname: ::prost::alloc::string::String,
    /// Only Verifier may update user's mobile number as it requires verification
    #[prost(message, optional, tag = "3")]
    pub mobile_number: ::core::option::Option<MobileNumber>,
}
/// Basic payment transaction with optional character appreciation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentTransactionV1 {
    /// account this tx is signed by
    #[prost(message, optional, tag = "1")]
    pub from: ::core::option::Option<AccountId>,
    /// dest is always a mobile number (of a user or a non-user
    #[prost(message, optional, tag = "2")]
    pub to: ::core::option::Option<MobileNumber>,
    /// amount in tokens to transfer
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<Amount>,
    /// network fee provided by sender
    #[prost(message, optional, tag = "4")]
    pub fee: ::core::option::Option<Amount>,
    /// optional extra tip to miners
    #[prost(message, optional, tag = "5")]
    pub tip: ::core::option::Option<Amount>,
    /// char trait set by sender. e.g. smart
    #[prost(enumeration = "CharTrait", tag = "6")]
    pub r#trait: i32,
}
/// new user transactions can be submitted by sms verifiers only
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewUserTransactionV1 {
    /// initial user balance
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionData {
    /// binary transaction data
    #[prost(bytes = "vec", tag = "1")]
    pub transaction_data: ::prost::alloc::vec::Vec<u8>,
    /// transaction type for deserialization
    #[prost(enumeration = "TransactionType", tag = "2")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionWithStatus {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<TransactionData>,
    /// transaction status
    #[prost(enumeration = "TransactionStatus", tag = "2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedTransaction {
    /// time transaction was signed
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    /// binary transaction data
    #[prost(message, optional, tag = "2")]
    pub transaction_data: ::core::option::Option<TransactionData>,
    /// network id to avoid confusion with testnets
    #[prost(uint32, tag = "3")]
    pub network_id: u32,
    /// signer signature on all of the above data
    #[prost(message, optional, tag = "4")]
    pub signature: ::core::option::Option<Signature>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewUserEvent {
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    #[prost(message, optional, tag = "2")]
    pub account_id: ::core::option::Option<AccountId>,
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionEvent {
    /// ledger height of execution
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(message, optional, tag = "2")]
    pub transaction: ::core::option::Option<SignedTransaction>,
    #[prost(enumeration = "ExecutionResult", tag = "3")]
    pub result: i32,
    #[prost(enumeration = "FeeType", tag = "4")]
    pub fee_type: i32,
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
    /// fee is minted by the protocol
    Mint = 0,
    /// fee is paid by the transaction signer
    User = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionResult {
    InvalidNonce = 0,
    InsufficientBalance = 1,
    Executed = 2,
}
