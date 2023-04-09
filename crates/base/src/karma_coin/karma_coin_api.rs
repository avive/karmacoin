#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCommunityAdminRequest {
    /// Caller serialized SetCommunityAdminData protobufs message
    #[prost(message, optional, tag = "1")]
    pub from_account_id: ::core::option::Option<super::core_types::AccountId>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Caller signature on the data
    #[prost(bytes = "vec", tag = "3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCommunityAdminData {
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    #[prost(message, optional, tag = "2")]
    pub target_account_id: ::core::option::Option<super::core_types::AccountId>,
    #[prost(uint32, tag = "3")]
    pub community_id: u32,
    #[prost(bool, tag = "4")]
    pub admin: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCommunityAdminResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLeaderBoardRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContactsRequest {
    /// optional prefix over user names for auto-complete purposes
    #[prost(string, tag = "1")]
    pub prefix: ::prost::alloc::string::String,
    /// optional filter by a community
    #[prost(uint32, tag = "2")]
    pub community_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContactsResponse {
    #[prost(message, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<super::core_types::Contact>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLeaderBoardResponse {
    #[prost(message, repeated, tag = "1")]
    pub leaderboard_entries: ::prost::alloc::vec::Vec<
        super::core_types::LeaderboardEntry,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsFromHashesRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub tx_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsFromHashesResponse {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<
        super::core_types::SignedTransactionWithStatus,
    >,
    #[prost(message, optional, tag = "2")]
    pub tx_events: ::core::option::Option<super::core_types::TransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllUsersRequest {
    #[prost(uint32, tag = "1")]
    pub community_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllUsersResponse {
    #[prost(message, repeated, tag = "1")]
    pub users: ::prost::alloc::vec::Vec<super::core_types::User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExchangeRateRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExchangeRateResponse {
    /// Estimated 1 KC value in USD
    #[prost(double, tag = "1")]
    pub exchange_rate: f64,
}
/// Get user by user name
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByUserNameRequest {
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByUserNameResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::core_types::User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTransactionRequest {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<super::core_types::SignedTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTransactionResponse {
    #[prost(enumeration = "SubmitTransactionResult", tag = "1")]
    pub submit_transaction_result: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByNumberRequest {
    #[prost(message, optional, tag = "1")]
    pub mobile_number: ::core::option::Option<super::core_types::MobileNumber>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByNumberResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::core_types::User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByAccountRequest {
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<super::core_types::AccountId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::core_types::User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGenesisDataRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGenesisDataResponse {
    #[prost(message, optional, tag = "1")]
    pub genesis_data: ::core::option::Option<super::core_types::GenesisData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockchainDataRequest {}
/// Current blockchain data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockchainDataResponse {
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<super::core_types::BlockchainStats>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsRequest {
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<super::core_types::AccountId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<
        super::core_types::SignedTransactionWithStatus,
    >,
    #[prost(message, optional, tag = "2")]
    pub tx_events: ::core::option::Option<super::core_types::TransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResponse {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<
        super::core_types::SignedTransactionWithStatus,
    >,
    #[prost(message, optional, tag = "2")]
    pub tx_events: ::core::option::Option<super::core_types::TransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockchainEventsRequest {
    #[prost(uint64, tag = "1")]
    pub from_block_height: u64,
    #[prost(uint64, tag = "2")]
    pub to_block_height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockchainEventsResponse {
    #[prost(message, repeated, tag = "1")]
    pub blocks_events: ::prost::alloc::vec::Vec<super::core_types::BlockEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksRequest {
    #[prost(uint64, tag = "1")]
    pub from_block_height: u64,
    #[prost(uint64, tag = "2")]
    pub to_block_height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksResponse {
    #[prost(message, repeated, tag = "1")]
    pub blocks: ::prost::alloc::vec::Vec<super::core_types::Block>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubmitTransactionResult {
    Rejected = 0,
    Submitted = 1,
}
impl SubmitTransactionResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubmitTransactionResult::Rejected => "SUBMIT_TRANSACTION_RESULT_REJECTED",
            SubmitTransactionResult::Submitted => "SUBMIT_TRANSACTION_RESULT_SUBMITTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBMIT_TRANSACTION_RESULT_REJECTED" => Some(Self::Rejected),
            "SUBMIT_TRANSACTION_RESULT_SUBMITTED" => Some(Self::Submitted),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod api_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Unified public API provided by blockchain nodes and verifiers
    #[derive(Debug, Clone)]
    pub struct ApiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ApiServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ApiServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ApiServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ApiServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// make another user a community admin. Can only be executed by an existing community admin
        /// in Karmachain 2.0 this will need to come from sudo
        pub async fn set_community_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::SetCommunityAdminRequest>,
        ) -> Result<tonic::Response<super::SetCommunityAdminResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/SetCommunityAdmin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// get current karma rewards period leaderboard
        pub async fn get_leader_board(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLeaderBoardRequest>,
        ) -> Result<tonic::Response<super::GetLeaderBoardResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetLeaderBoard",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// return all accounts on chain - doesn't support pagination yet but should
        pub async fn get_all_users(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllUsersRequest>,
        ) -> Result<tonic::Response<super::GetAllUsersResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetAllUsers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// get contacts for easy appreciation from app of any existing user. should support pagination and prefix filtering
        pub async fn get_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetContactsRequest>,
        ) -> Result<tonic::Response<super::GetContactsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetContacts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Return transactions for provided one or more tx hash
        pub async fn get_transactions_from_hashes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsFromHashesRequest>,
        ) -> Result<
            tonic::Response<super::GetTransactionsFromHashesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetTransactionsFromHashes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// check if a username is available
        pub async fn get_user_info_by_user_name(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserInfoByUserNameRequest>,
        ) -> Result<
            tonic::Response<super::GetUserInfoByUserNameResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetUserInfoByUserName",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns on-chain user info by phone number if user exists
        pub async fn get_user_info_by_number(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserInfoByNumberRequest>,
        ) -> Result<tonic::Response<super::GetUserInfoByNumberResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetUserInfoByNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns on-chain user info by account id if user exists
        pub async fn get_user_info_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserInfoByAccountRequest>,
        ) -> Result<
            tonic::Response<super::GetUserInfoByAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetUserInfoByAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the current blockchain state
        pub async fn get_blockchain_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockchainDataRequest>,
        ) -> Result<tonic::Response<super::GetBlockchainDataResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetBlockchainData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the current blockchain state
        pub async fn get_genesis_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGenesisDataRequest>,
        ) -> Result<tonic::Response<super::GetGenesisDataResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetGenesisData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Submit a signed transaction to the blockchain
        pub async fn submit_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitTransactionRequest>,
        ) -> Result<tonic::Response<super::SubmitTransactionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/SubmitTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get all transactions between two account, included transactions in the pool and not yet on-chain
        /// Results include txs current status and all events omitted for each transaction
        pub async fn get_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsRequest>,
        ) -> Result<tonic::Response<super::GetTransactionsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get transaction data by its digest hash. Transaction may be in pool or on-chain
        /// Returns all events associated with the transaction
        pub async fn get_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get blockchain events for a range of heights
        pub async fn get_blockchain_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockchainEventsRequest>,
        ) -> Result<tonic::Response<super::GetBlockchainEventsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetBlockchainEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get blockchain events for a range of heights
        pub async fn get_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksRequest>,
        ) -> Result<tonic::Response<super::GetBlocksResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetBlocks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod api_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ApiServiceServer.
    #[async_trait]
    pub trait ApiService: Send + Sync + 'static {
        /// make another user a community admin. Can only be executed by an existing community admin
        /// in Karmachain 2.0 this will need to come from sudo
        async fn set_community_admin(
            &self,
            request: tonic::Request<super::SetCommunityAdminRequest>,
        ) -> Result<tonic::Response<super::SetCommunityAdminResponse>, tonic::Status>;
        /// get current karma rewards period leaderboard
        async fn get_leader_board(
            &self,
            request: tonic::Request<super::GetLeaderBoardRequest>,
        ) -> Result<tonic::Response<super::GetLeaderBoardResponse>, tonic::Status>;
        /// return all accounts on chain - doesn't support pagination yet but should
        async fn get_all_users(
            &self,
            request: tonic::Request<super::GetAllUsersRequest>,
        ) -> Result<tonic::Response<super::GetAllUsersResponse>, tonic::Status>;
        /// get contacts for easy appreciation from app of any existing user. should support pagination and prefix filtering
        async fn get_contacts(
            &self,
            request: tonic::Request<super::GetContactsRequest>,
        ) -> Result<tonic::Response<super::GetContactsResponse>, tonic::Status>;
        /// Return transactions for provided one or more tx hash
        async fn get_transactions_from_hashes(
            &self,
            request: tonic::Request<super::GetTransactionsFromHashesRequest>,
        ) -> Result<
            tonic::Response<super::GetTransactionsFromHashesResponse>,
            tonic::Status,
        >;
        /// check if a username is available
        async fn get_user_info_by_user_name(
            &self,
            request: tonic::Request<super::GetUserInfoByUserNameRequest>,
        ) -> Result<
            tonic::Response<super::GetUserInfoByUserNameResponse>,
            tonic::Status,
        >;
        /// Returns on-chain user info by phone number if user exists
        async fn get_user_info_by_number(
            &self,
            request: tonic::Request<super::GetUserInfoByNumberRequest>,
        ) -> Result<tonic::Response<super::GetUserInfoByNumberResponse>, tonic::Status>;
        /// Returns on-chain user info by account id if user exists
        async fn get_user_info_by_account(
            &self,
            request: tonic::Request<super::GetUserInfoByAccountRequest>,
        ) -> Result<tonic::Response<super::GetUserInfoByAccountResponse>, tonic::Status>;
        /// Returns the current blockchain state
        async fn get_blockchain_data(
            &self,
            request: tonic::Request<super::GetBlockchainDataRequest>,
        ) -> Result<tonic::Response<super::GetBlockchainDataResponse>, tonic::Status>;
        /// Returns the current blockchain state
        async fn get_genesis_data(
            &self,
            request: tonic::Request<super::GetGenesisDataRequest>,
        ) -> Result<tonic::Response<super::GetGenesisDataResponse>, tonic::Status>;
        /// Submit a signed transaction to the blockchain
        async fn submit_transaction(
            &self,
            request: tonic::Request<super::SubmitTransactionRequest>,
        ) -> Result<tonic::Response<super::SubmitTransactionResponse>, tonic::Status>;
        /// Get all transactions between two account, included transactions in the pool and not yet on-chain
        /// Results include txs current status and all events omitted for each transaction
        async fn get_transactions(
            &self,
            request: tonic::Request<super::GetTransactionsRequest>,
        ) -> Result<tonic::Response<super::GetTransactionsResponse>, tonic::Status>;
        /// Get transaction data by its digest hash. Transaction may be in pool or on-chain
        /// Returns all events associated with the transaction
        async fn get_transaction(
            &self,
            request: tonic::Request<super::GetTransactionRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status>;
        /// Get blockchain events for a range of heights
        async fn get_blockchain_events(
            &self,
            request: tonic::Request<super::GetBlockchainEventsRequest>,
        ) -> Result<tonic::Response<super::GetBlockchainEventsResponse>, tonic::Status>;
        /// Get blockchain events for a range of heights
        async fn get_blocks(
            &self,
            request: tonic::Request<super::GetBlocksRequest>,
        ) -> Result<tonic::Response<super::GetBlocksResponse>, tonic::Status>;
    }
    /// Unified public API provided by blockchain nodes and verifiers
    #[derive(Debug)]
    pub struct ApiServiceServer<T: ApiService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ApiService> ApiServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ApiServiceServer<T>
    where
        T: ApiService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/karma_coin.api.ApiService/SetCommunityAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct SetCommunityAdminSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::SetCommunityAdminRequest>
                    for SetCommunityAdminSvc<T> {
                        type Response = super::SetCommunityAdminResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetCommunityAdminRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_community_admin(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetCommunityAdminSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetLeaderBoard" => {
                    #[allow(non_camel_case_types)]
                    struct GetLeaderBoardSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetLeaderBoardRequest>
                    for GetLeaderBoardSvc<T> {
                        type Response = super::GetLeaderBoardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLeaderBoardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_leader_board(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLeaderBoardSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetAllUsers" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllUsersSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetAllUsersRequest>
                    for GetAllUsersSvc<T> {
                        type Response = super::GetAllUsersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAllUsersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_all_users(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllUsersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetContacts" => {
                    #[allow(non_camel_case_types)]
                    struct GetContactsSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetContactsRequest>
                    for GetContactsSvc<T> {
                        type Response = super::GetContactsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetContactsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_contacts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetContactsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetTransactionsFromHashes" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsFromHashesSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<
                        super::GetTransactionsFromHashesRequest,
                    > for GetTransactionsFromHashesSvc<T> {
                        type Response = super::GetTransactionsFromHashesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetTransactionsFromHashesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_transactions_from_hashes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionsFromHashesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetUserInfoByUserName" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserInfoByUserNameSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetUserInfoByUserNameRequest>
                    for GetUserInfoByUserNameSvc<T> {
                        type Response = super::GetUserInfoByUserNameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserInfoByUserNameRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_info_by_user_name(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserInfoByUserNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetUserInfoByNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserInfoByNumberSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetUserInfoByNumberRequest>
                    for GetUserInfoByNumberSvc<T> {
                        type Response = super::GetUserInfoByNumberResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserInfoByNumberRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_info_by_number(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserInfoByNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetUserInfoByAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserInfoByAccountSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetUserInfoByAccountRequest>
                    for GetUserInfoByAccountSvc<T> {
                        type Response = super::GetUserInfoByAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserInfoByAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_info_by_account(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserInfoByAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetBlockchainData" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockchainDataSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetBlockchainDataRequest>
                    for GetBlockchainDataSvc<T> {
                        type Response = super::GetBlockchainDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockchainDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_blockchain_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockchainDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetGenesisData" => {
                    #[allow(non_camel_case_types)]
                    struct GetGenesisDataSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetGenesisDataRequest>
                    for GetGenesisDataSvc<T> {
                        type Response = super::GetGenesisDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGenesisDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_genesis_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGenesisDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/SubmitTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitTransactionSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::SubmitTransactionRequest>
                    for SubmitTransactionSvc<T> {
                        type Response = super::SubmitTransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).submit_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetTransactions" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetTransactionsRequest>
                    for GetTransactionsSvc<T> {
                        type Response = super::GetTransactionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_transactions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetTransactionRequest>
                    for GetTransactionSvc<T> {
                        type Response = super::GetTransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetBlockchainEvents" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockchainEventsSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetBlockchainEventsRequest>
                    for GetBlockchainEventsSvc<T> {
                        type Response = super::GetBlockchainEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockchainEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_blockchain_events(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockchainEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlocksSvc<T: ApiService>(pub Arc<T>);
                    impl<
                        T: ApiService,
                    > tonic::server::UnaryService<super::GetBlocksRequest>
                    for GetBlocksSvc<T> {
                        type Response = super::GetBlocksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ApiService> Clone for ApiServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ApiService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ApiService> tonic::server::NamedService for ApiServiceServer<T> {
        const NAME: &'static str = "karma_coin.api.ApiService";
    }
}
