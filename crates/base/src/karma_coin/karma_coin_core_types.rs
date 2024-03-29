/// Derived from a public key
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountId {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(uint64, tag = "1")]
    pub free: u64,
    #[prost(uint64, tag = "2")]
    pub reserved: u64,
    #[prost(uint64, tag = "3")]
    pub misc_frozen: u64,
    #[prost(uint64, tag = "4")]
    pub fee_frozen: u64,
}
/// An public encryption key
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreKey {
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<PublicKey>,
    #[prost(uint32, tag = "2")]
    pub id: u32,
    #[prost(enumeration = "KeyScheme", tag = "3")]
    pub scheme: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyPair {
    #[prost(message, optional, tag = "1")]
    pub private_key: ::core::option::Option<PrivateKey>,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<PublicKey>,
    #[prost(enumeration = "KeyScheme", tag = "3")]
    pub scheme: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(enumeration = "KeyScheme", tag = "1")]
    pub scheme: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileNumber {
    /// always up to 12 digits which including country code
    #[prost(string, tag = "1")]
    pub number: ::prost::alloc::string::String,
}
/// user on-chain data
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
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
    #[prost(uint64, tag = "5")]
    pub balance: u64,
    #[prost(message, repeated, tag = "6")]
    pub trait_scores: ::prost::alloc::vec::Vec<TraitScore>,
    /// one-time enc pre-keys for e2e messaging
    #[prost(message, repeated, tag = "7")]
    pub pre_keys: ::prost::alloc::vec::Vec<PreKey>,
    /// user's current karma score
    #[prost(uint32, tag = "8")]
    pub karma_score: u32,
    /// all communities user is member of, admin rights and score in each
    #[prost(message, repeated, tag = "9")]
    pub community_memberships: ::prost::alloc::vec::Vec<CommunityMembership>,
}
/// Contact information for easy appreciation in the app of any contact
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub account_id: ::core::option::Option<AccountId>,
    #[prost(message, optional, tag = "3")]
    pub mobile_number: ::core::option::Option<MobileNumber>,
    #[prost(message, repeated, tag = "4")]
    pub community_memberships: ::prost::alloc::vec::Vec<CommunityMembership>,
    #[prost(message, repeated, tag = "5")]
    pub trait_scores: ::prost::alloc::vec::Vec<TraitScore>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityMembership {
    #[prost(uint32, tag = "1")]
    pub community_id: u32,
    #[prost(uint32, tag = "2")]
    pub karma_score: u32,
    /// when true user is admin of the community - set by sudo
    #[prost(bool, tag = "3")]
    pub is_admin: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderboardEntry {
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub account_id: ::core::option::Option<AccountId>,
    /// score in leaderboard
    #[prost(uint32, tag = "4")]
    pub score: u32,
    /// outgoing appreciations char traits
    #[prost(uint32, repeated, tag = "5")]
    pub char_traits_ids: ::prost::alloc::vec::Vec<u32>,
}
/// Phone verifier is an entity that verifies account mobile phone numbers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoneVerifier {
    /// verifier account id
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<AccountId>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
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
    #[prost(uint64, tag = "5")]
    pub fees: u64,
    /// digest of block in consensus at the previous height
    #[prost(bytes = "vec", tag = "6")]
    pub prev_block_digest: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "7")]
    pub signature: ::core::option::Option<Signature>,
    #[prost(uint64, tag = "8")]
    pub reward: u64,
    /// total coins minted in this block (rewards + tx fee subsidies)
    #[prost(uint64, tag = "9")]
    pub minted: u64,
    /// block digest includes hash of all above data
    #[prost(bytes = "vec", tag = "10")]
    pub digest: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharTrait {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub emoji: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraitScore {
    #[prost(uint32, tag = "1")]
    pub trait_id: u32,
    #[prost(uint32, tag = "2")]
    pub score: u32,
    /// 0 for no community, otherwise community id this trait was assigned
    #[prost(uint32, tag = "3")]
    pub community_id: u32,
}
/// a community partner
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Community {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub emoji: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub website_url: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub twitter_url: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub insta_url: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub face_url: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub discord_url: ::prost::alloc::string::String,
    #[prost(uint32, repeated, tag = "10")]
    pub char_trait_ids: ::prost::alloc::vec::Vec<u32>,
    /// closed community - only community manager can invite new members
    /// and only members can appreciate each other in the community
    #[prost(bool, tag = "11")]
    pub closed: bool,
}
/// new user transactions submitted by users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewUserTransactionV1 {
    /// Evidence from a valid verifier about the new user
    #[prost(message, optional, tag = "1")]
    pub verify_number_response: ::core::option::Option<UserVerificationData>,
}
/// new user transactions submitted by users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewUserTransactionV2 {
    /// Serialized UserVerifcationDataEx evidence from a valid verifier about the new user
    #[prost(bytes = "vec", tag = "1")]
    pub user_verification_data_ex: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub verifier_signature: ::core::option::Option<Signature>,
}
/// Basic payment transaction with optional character appreciation
/// Receiver must be identified by phone number or a karma coin account id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentTransactionV1 {
    /// must be included so it is part of signed message and part of the tx hash
    #[prost(message, optional, tag = "1")]
    pub from: ::core::option::Option<AccountId>,
    /// amount in tokens to transfer
    #[prost(uint64, tag = "2")]
    pub amount: u64,
    /// IMPORTANT: one of the next fields must be provided as the payee for the tx to be valid
    /// todo: use oneof proto feature here
    ///
    /// pay to a mobile number
    #[prost(message, optional, tag = "3")]
    pub to_number: ::core::option::Option<MobileNumber>,
    /// pay to a Karma Coin account id
    #[prost(message, optional, tag = "4")]
    pub to_account_id: ::core::option::Option<AccountId>,
    /// char trait id set by sender. e.g. smart
    #[prost(uint32, tag = "5")]
    pub char_trait_id: u32,
    #[prost(uint32, tag = "6")]
    pub community_id: u32,
}
/// Update user info
/// User can update his nickname, mobile phone number or accountId in case
/// he wants to associate his number with a new accountId.
/// in that case, only way to transact from old account with old account id is via
/// payment transactions from an accountId signed by this accountId private key
/// Also in this case a new onchain account should be created with the new accountId
/// and the user's phone number is going to be assoicated with the new account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserTransactionV1 {
    /// new requested nickname
    #[prost(string, tag = "1")]
    pub nickname: ::prost::alloc::string::String,
    /// Updated mobile number or existing one
    #[prost(message, optional, tag = "2")]
    pub mobile_number: ::core::option::Option<MobileNumber>,
    /// verifier attestation regarding the number and the account
    #[prost(message, optional, tag = "3")]
    pub user_verification_data: ::core::option::Option<UserVerificationData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserTransactionV1 {}
/// The generic transaction payload - unsigned
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBody {
    /// time transaction was signed
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    /// tx nonce
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
    /// network fee provided by sender
    #[prost(uint64, tag = "3")]
    pub fee: u64,
    /// binary transaction data
    #[prost(message, optional, tag = "4")]
    pub transaction_data: ::core::option::Option<TransactionData>,
    /// network id to avoid confusion with testnets
    #[prost(uint32, tag = "5")]
    pub net_id: u32,
}
/// serialized transaction data
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionData {
    /// binary transaction data (e.g. NewUserTxV1, PaymentV1, etc...)
    #[prost(bytes = "vec", tag = "1")]
    pub transaction_data: ::prost::alloc::vec::Vec<u8>,
    /// transaction type for deserialization
    #[prost(enumeration = "TransactionType", tag = "2")]
    pub transaction_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedTransaction {
    /// account this tx is signed by
    #[prost(message, optional, tag = "1")]
    pub signer: ::core::option::Option<AccountId>,
    /// Transaction data
    #[prost(bytes = "vec", tag = "2")]
    pub transaction_body: ::prost::alloc::vec::Vec<u8>,
    /// signer signature on all of the above data
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<Signature>,
}
/// Created and signed by a verifier to attest that an account owns a mobile number
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserVerificationData {
    #[prost(message, optional, tag = "1")]
    pub verifier_account_id: ::core::option::Option<AccountId>,
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(enumeration = "VerificationResult", tag = "3")]
    pub verification_result: i32,
    #[prost(message, optional, tag = "4")]
    pub account_id: ::core::option::Option<AccountId>,
    #[prost(message, optional, tag = "5")]
    pub mobile_number: ::core::option::Option<MobileNumber>,
    #[prost(string, tag = "6")]
    pub requested_user_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub signature: ::core::option::Option<Signature>,
}
/// Created and signed by a verifier to attest that an account owns a mobile number
/// Includes mobile number hash instead of mobile number in response
/// Signature is externally available
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserVerificationDataEx {
    #[prost(message, optional, tag = "1")]
    pub verifier_account_id: ::core::option::Option<AccountId>,
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(enumeration = "VerificationResult", tag = "3")]
    pub verification_result: i32,
    #[prost(message, optional, tag = "4")]
    pub account_id: ::core::option::Option<AccountId>,
    #[prost(string, tag = "5")]
    pub mobile_number_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub requested_user_name: ::prost::alloc::string::String,
}
/// a collection of signed transactions
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedTransactionsHashes {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Pending transactions are transactions that are not yet on chain
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemPool {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<SignedTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedTransactionWithStatus {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<SignedTransaction>,
    /// transaction status
    #[prost(enumeration = "TransactionStatus", tag = "2")]
    pub status: i32,
    /// sender
    #[prost(message, optional, tag = "3")]
    pub from: ::core::option::Option<User>,
    /// receiver
    #[prost(message, optional, tag = "4")]
    pub to: ::core::option::Option<User>,
}
/// Transaction added to ledger
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
    #[prost(enumeration = "ExecutionInfo", tag = "6")]
    pub info: i32,
    #[prost(string, tag = "7")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(enumeration = "FeeType", tag = "8")]
    pub fee_type: i32,
    #[prost(uint64, tag = "9")]
    pub signup_reward: u64,
    #[prost(uint64, tag = "10")]
    pub referral_reward: u64,
    #[prost(uint32, tag = "11")]
    pub appreciation_char_trait_idx: u32,
    #[prost(uint32, tag = "12")]
    pub appreciation_community_id: u32,
    #[prost(uint64, tag = "13")]
    pub fee: u64,
}
/// A collection of events for a transaction
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<TransactionEvent>,
}
/// Blockchain aggregated data
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
    /// total number of payment transactions with an appreciation
    #[prost(uint64, tag = "5")]
    pub appreciations_transactions_count: u64,
    /// total number of verified user accounts
    #[prost(uint64, tag = "6")]
    pub users_count: u64,
    /// total tx fees collected by block producers
    #[prost(uint64, tag = "7")]
    pub fees_amount: u64,
    /// total number of kCents minted by the protocol since genesis
    #[prost(uint64, tag = "8")]
    pub minted_amount: u64,
    /// total number of kCents in circulation by minting. Not including pre-mint
    #[prost(uint64, tag = "9")]
    pub circulation: u64,
    /// total tx fee subsidies issued by the protocol
    #[prost(uint64, tag = "10")]
    pub fee_subs_count: u64,
    #[prost(uint64, tag = "11")]
    pub fee_subs_amount: u64,
    #[prost(uint64, tag = "12")]
    pub signup_rewards_count: u64,
    #[prost(uint64, tag = "13")]
    pub signup_rewards_amount: u64,
    #[prost(uint64, tag = "14")]
    pub referral_rewards_count: u64,
    #[prost(uint64, tag = "15")]
    pub referral_rewards_amount: u64,
    #[prost(uint64, tag = "16")]
    pub validator_rewards_count: u64,
    #[prost(uint64, tag = "17")]
    pub validator_rewards_amount: u64,
    /// total number of payment transactions
    #[prost(uint64, tag = "18")]
    pub update_user_transactions_count: u64,
    /// estimated KC to USD exchange rate
    #[prost(double, tag = "19")]
    pub exchange_rate: f64,
    /// amount of rewards paid to causes
    #[prost(uint64, tag = "20")]
    pub causes_rewards_amount: u64,
    /// karma rewards given
    #[prost(uint64, tag = "21")]
    pub karma_rewards_count: u64,
    /// karma rewards amount allocated
    #[prost(uint64, tag = "22")]
    pub karma_rewards_amount: u64,
}
/// Block events
#[allow(clippy::derive_partial_eq_without_eq)]
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
    pub signups_count: u64,
    #[prost(uint64, tag = "6")]
    pub payments_count: u64,
    #[prost(uint64, tag = "7")]
    pub appreciations_count: u64,
    #[prost(uint64, tag = "8")]
    pub user_updates_count: u64,
    #[prost(uint64, tag = "9")]
    pub fees_amount: u64,
    #[prost(uint64, tag = "10")]
    pub signup_rewards_amount: u64,
    #[prost(uint64, tag = "11")]
    pub referral_rewards_amount: u64,
    #[prost(uint64, tag = "12")]
    pub referral_rewards_count: u64,
    #[prost(uint64, tag = "13")]
    pub reward: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisData {
    #[prost(uint32, tag = "1")]
    pub net_id: u32,
    #[prost(string, tag = "2")]
    pub net_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub genesis_time: u64,
    #[prost(uint64, tag = "4")]
    pub signup_reward_phase1_alloc: u64,
    #[prost(uint64, tag = "5")]
    pub signup_reward_phase2_alloc: u64,
    #[prost(uint64, tag = "6")]
    pub signup_reward_phase1_amount: u64,
    #[prost(uint64, tag = "7")]
    pub signup_reward_phase2_amount: u64,
    #[prost(uint64, tag = "8")]
    pub signup_reward_phase3_start: u64,
    #[prost(uint64, tag = "9")]
    pub referral_reward_phase1_alloc: u64,
    #[prost(uint64, tag = "10")]
    pub referral_reward_phase2_alloc: u64,
    #[prost(uint64, tag = "11")]
    pub referral_reward_phase1_amount: u64,
    #[prost(uint64, tag = "12")]
    pub referral_reward_phase2_amount: u64,
    #[prost(uint64, tag = "13")]
    pub tx_fee_subsidy_max_per_user: u64,
    #[prost(uint64, tag = "14")]
    pub tx_fee_subsidies_alloc: u64,
    #[prost(uint64, tag = "15")]
    pub tx_fee_subsidy_max_amount: u64,
    #[prost(uint64, tag = "16")]
    pub block_reward_amount: u64,
    #[prost(uint64, tag = "17")]
    pub block_reward_last_block: u64,
    #[prost(uint64, tag = "18")]
    pub karma_reward_amount: u64,
    #[prost(uint64, tag = "19")]
    pub karma_reward_alloc: u64,
    #[prost(uint64, tag = "20")]
    pub karma_reward_top_n_users: u64,
    #[prost(uint64, tag = "21")]
    pub karma_rewards_eligibility: u64,
    #[prost(uint64, tag = "22")]
    pub karma_rewards_period_hours: u64,
    #[prost(uint64, tag = "23")]
    pub validators_pool_amount: u64,
    #[prost(string, tag = "24")]
    pub validators_pool_account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "25")]
    pub validators_pool_account_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "26")]
    pub char_traits: ::prost::alloc::vec::Vec<CharTrait>,
    #[prost(message, repeated, tag = "27")]
    pub verifiers: ::prost::alloc::vec::Vec<PhoneVerifier>,
}
/// Supported signature schemes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyScheme {
    Ed25519 = 0,
}
impl KeyScheme {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyScheme::Ed25519 => "KEY_SCHEME_ED25519",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KEY_SCHEME_ED25519" => Some(Self::Ed25519),
            _ => None,
        }
    }
}
/// / transactions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionType {
    PaymentV1 = 0,
    NewUserV1 = 1,
    UpdateUserV1 = 2,
    DeleteUserV1 = 3,
    DeleteUserV = 4,
}
impl TransactionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionType::PaymentV1 => "TRANSACTION_TYPE_PAYMENT_V1",
            TransactionType::NewUserV1 => "TRANSACTION_TYPE_NEW_USER_V1",
            TransactionType::UpdateUserV1 => "TRANSACTION_TYPE_UPDATE_USER_V1",
            TransactionType::DeleteUserV1 => "TRANSACTION_TYPE_DELETE_USER_V1",
            TransactionType::DeleteUserV => "TRANSACTION_TYPE_DELETE_USER_V",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSACTION_TYPE_PAYMENT_V1" => Some(Self::PaymentV1),
            "TRANSACTION_TYPE_NEW_USER_V1" => Some(Self::NewUserV1),
            "TRANSACTION_TYPE_UPDATE_USER_V1" => Some(Self::UpdateUserV1),
            "TRANSACTION_TYPE_DELETE_USER_V1" => Some(Self::DeleteUserV1),
            "TRANSACTION_TYPE_DELETE_USER_V" => Some(Self::DeleteUserV),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VerificationResult {
    Unspecified = 0,
    /// there's already a user with the requested user name
    UserNameTaken = 1,
    /// user is verified using provided token
    Verified = 2,
    /// user is not verifier using provided token
    Unverified = 3,
    /// request is missing required data
    MissingData = 4,
    /// bad client signature
    InvalidSignature = 5,
    /// different account associated with phone number
    AccountMismatch = 6,
}
impl VerificationResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VerificationResult::Unspecified => "VERIFICATION_RESULT_UNSPECIFIED",
            VerificationResult::UserNameTaken => "VERIFICATION_RESULT_USER_NAME_TAKEN",
            VerificationResult::Verified => "VERIFICATION_RESULT_VERIFIED",
            VerificationResult::Unverified => "VERIFICATION_RESULT_UNVERIFIED",
            VerificationResult::MissingData => "VERIFICATION_RESULT_MISSING_DATA",
            VerificationResult::InvalidSignature => {
                "VERIFICATION_RESULT_INVALID_SIGNATURE"
            }
            VerificationResult::AccountMismatch => "VERIFICATION_RESULT_ACCOUNT_MISMATCH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VERIFICATION_RESULT_UNSPECIFIED" => Some(Self::Unspecified),
            "VERIFICATION_RESULT_USER_NAME_TAKEN" => Some(Self::UserNameTaken),
            "VERIFICATION_RESULT_VERIFIED" => Some(Self::Verified),
            "VERIFICATION_RESULT_UNVERIFIED" => Some(Self::Unverified),
            "VERIFICATION_RESULT_MISSING_DATA" => Some(Self::MissingData),
            "VERIFICATION_RESULT_INVALID_SIGNATURE" => Some(Self::InvalidSignature),
            "VERIFICATION_RESULT_ACCOUNT_MISMATCH" => Some(Self::AccountMismatch),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionStatus {
    Unknown = 0,
    NotSubmitted = 1,
    Submitted = 2,
    Rejected = 3,
    OnChain = 4,
}
impl TransactionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionStatus::Unknown => "TRANSACTION_STATUS_UNKNOWN",
            TransactionStatus::NotSubmitted => "TRANSACTION_STATUS_NOT_SUBMITTED",
            TransactionStatus::Submitted => "TRANSACTION_STATUS_SUBMITTED",
            TransactionStatus::Rejected => "TRANSACTION_STATUS_REJECTED",
            TransactionStatus::OnChain => "TRANSACTION_STATUS_ON_CHAIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSACTION_STATUS_UNKNOWN" => Some(Self::Unknown),
            "TRANSACTION_STATUS_NOT_SUBMITTED" => Some(Self::NotSubmitted),
            "TRANSACTION_STATUS_SUBMITTED" => Some(Self::Submitted),
            "TRANSACTION_STATUS_REJECTED" => Some(Self::Rejected),
            "TRANSACTION_STATUS_ON_CHAIN" => Some(Self::OnChain),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FeeType {
    /// fee provided by the protocol
    Mint = 0,
    /// fee provided by the transaction signer
    User = 1,
}
impl FeeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FeeType::Mint => "FEE_TYPE_MINT",
            FeeType::User => "FEE_TYPE_USER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FEE_TYPE_MINT" => Some(Self::Mint),
            "FEE_TYPE_USER" => Some(Self::User),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionResult {
    Executed = 0,
    /// invalid syntax
    Invalid = 1,
}
impl ExecutionResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionResult::Executed => "EXECUTION_RESULT_EXECUTED",
            ExecutionResult::Invalid => "EXECUTION_RESULT_INVALID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_RESULT_EXECUTED" => Some(Self::Executed),
            "EXECUTION_RESULT_INVALID" => Some(Self::Invalid),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionInfo {
    Unknown = 0,
    NicknameUpdated = 1,
    NicknameNotAvailable = 2,
    NicknameInvalid = 3,
    NumberUpdated = 4,
    AccountCreated = 5,
    PaymentConfirmed = 6,
    InvalidData = 7,
    AccountAlreadyExists = 8,
    TxFeeTooLow = 9,
    InternalNodeError = 10,
}
impl ExecutionInfo {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionInfo::Unknown => "EXECUTION_INFO_UNKNOWN",
            ExecutionInfo::NicknameUpdated => "EXECUTION_INFO_NICKNAME_UPDATED",
            ExecutionInfo::NicknameNotAvailable => {
                "EXECUTION_INFO_NICKNAME_NOT_AVAILABLE"
            }
            ExecutionInfo::NicknameInvalid => "EXECUTION_INFO_NICKNAME_INVALID",
            ExecutionInfo::NumberUpdated => "EXECUTION_INFO_NUMBER_UPDATED",
            ExecutionInfo::AccountCreated => "EXECUTION_INFO_ACCOUNT_CREATED",
            ExecutionInfo::PaymentConfirmed => "EXECUTION_INFO_PAYMENT_CONFIRMED",
            ExecutionInfo::InvalidData => "EXECUTION_INFO_INVALID_DATA",
            ExecutionInfo::AccountAlreadyExists => {
                "EXECUTION_INFO_ACCOUNT_ALREADY_EXISTS"
            }
            ExecutionInfo::TxFeeTooLow => "EXECUTION_INFO_TX_FEE_TOO_LOW",
            ExecutionInfo::InternalNodeError => "EXECUTION_INFO_INTERNAL_NODE_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_INFO_UNKNOWN" => Some(Self::Unknown),
            "EXECUTION_INFO_NICKNAME_UPDATED" => Some(Self::NicknameUpdated),
            "EXECUTION_INFO_NICKNAME_NOT_AVAILABLE" => Some(Self::NicknameNotAvailable),
            "EXECUTION_INFO_NICKNAME_INVALID" => Some(Self::NicknameInvalid),
            "EXECUTION_INFO_NUMBER_UPDATED" => Some(Self::NumberUpdated),
            "EXECUTION_INFO_ACCOUNT_CREATED" => Some(Self::AccountCreated),
            "EXECUTION_INFO_PAYMENT_CONFIRMED" => Some(Self::PaymentConfirmed),
            "EXECUTION_INFO_INVALID_DATA" => Some(Self::InvalidData),
            "EXECUTION_INFO_ACCOUNT_ALREADY_EXISTS" => Some(Self::AccountAlreadyExists),
            "EXECUTION_INFO_TX_FEE_TOO_LOW" => Some(Self::TxFeeTooLow),
            "EXECUTION_INFO_INTERNAL_NODE_ERROR" => Some(Self::InternalNodeError),
            _ => None,
        }
    }
}
