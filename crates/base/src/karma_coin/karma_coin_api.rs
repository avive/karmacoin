#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByNickRequest {
    #[prost(string, tag = "1")]
    pub nickname: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByNickResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::core_types::User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTransactionRequest {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<super::core_types::SignedTransaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTransactionResponse {
    #[prost(enumeration = "SubmitTransactionResult", tag = "1")]
    pub submit_transaction_result: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByNumberRequest {
    #[prost(message, optional, tag = "1")]
    pub mobile_number: ::core::option::Option<super::core_types::MobileNumber>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByNumberResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::core_types::User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByAccountRequest {
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<super::core_types::AccountId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoByAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::core_types::User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhoneVerifiersRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhoneVerifiersResponse {
    #[prost(message, repeated, tag = "1")]
    pub verifiers: ::prost::alloc::vec::Vec<super::core_types::PhoneVerifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCharTraitsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCharTraitsResponse {
    #[prost(message, repeated, tag = "1")]
    pub trait_names: ::prost::alloc::vec::Vec<super::core_types::TraitName>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGenesisDataRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGenesisDataResponse {
    /// from genesis
    #[prost(uint32, tag = "1")]
    pub network_id: u32,
    /// the provided API semantic version
    #[prost(string, tag = "2")]
    pub api_version: ::prost::alloc::string::String,
    /// from genesis
    #[prost(uint64, tag = "3")]
    pub genesis_time: u64,
    /// from genesis
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockchainDataRequest {}
/// Current blockchain data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockchainDataResponse {
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<super::core_types::BlockchainStats>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsRequest {
    #[prost(message, optional, tag = "1")]
    pub account_from: ::core::option::Option<super::core_types::AccountId>,
    #[prost(message, optional, tag = "2")]
    pub account_to: ::core::option::Option<super::core_types::AccountId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<super::core_types::SignedTransactionWithStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub digest: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResponse {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<super::core_types::SignedTransactionWithStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockchainEventsRequest {
    #[prost(uint64, tag = "1")]
    pub from_block_number: u64,
    #[prost(uint64, tag = "2")]
    pub to_block_number: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockchainEventsResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubmitTransactionResult {
    Invalid = 0,
    Submitted = 1,
}
#[doc = r" Generated client implementations."]
pub mod api_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Unified public API provided by blockchain nodes and verifiers"]
    #[derive(Debug, Clone)]
    pub struct ApiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ApiServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
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
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ApiServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ApiServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " check if a nickname is available"]
        pub async fn get_user_info_by_nick(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserInfoByNickRequest>,
        ) -> Result<tonic::Response<super::GetUserInfoByNickResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetUserInfoByNick",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns on-chain user info by phone number if user exists"]
        pub async fn get_user_info_by_number(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserInfoByNumberRequest>,
        ) -> Result<tonic::Response<super::GetUserInfoByNumberResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Returns on-chain user info by account id if user exists"]
        pub async fn get_user_info_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserInfoByAccountRequest>,
        ) -> Result<tonic::Response<super::GetUserInfoByAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Returns the identity of all phone verifiers registered on chain"]
        pub async fn get_phone_verifiers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPhoneVerifiersRequest>,
        ) -> Result<tonic::Response<super::GetPhoneVerifiersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.api.ApiService/GetPhoneVerifiers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all char traits on-chain"]
        pub async fn get_char_traits(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCharTraitsRequest>,
        ) -> Result<tonic::Response<super::GetCharTraitsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/karma_coin.api.ApiService/GetCharTraits");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the current blockchain state"]
        pub async fn get_blockchain_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockchainDataRequest>,
        ) -> Result<tonic::Response<super::GetBlockchainDataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Returns the current blockchain state"]
        pub async fn get_genesis_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGenesisDataRequest>,
        ) -> Result<tonic::Response<super::GetGenesisDataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/karma_coin.api.ApiService/GetGenesisData");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Submit a signed transaction to the blockchain"]
        pub async fn submit_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitTransactionRequest>,
        ) -> Result<tonic::Response<super::SubmitTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Get all transactions between two account, included transactions in the pool and not yet on-chain"]
        #[doc = " Results include txs current status"]
        pub async fn get_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsRequest>,
        ) -> Result<tonic::Response<super::GetTransactionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/karma_coin.api.ApiService/GetTransactions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get transaction data by its digest hash. Transaction may be in pool or on-chain"]
        pub async fn get_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/karma_coin.api.ApiService/GetTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get execution events for one or more blocks"]
        pub async fn get_blockchain_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockchainEventsRequest>,
        ) -> Result<tonic::Response<super::GetBlockchainEventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
    }
}
#[doc = r" Generated server implementations."]
pub mod api_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ApiServiceServer."]
    #[async_trait]
    pub trait ApiService: Send + Sync + 'static {
        #[doc = " check if a nickname is available"]
        async fn get_user_info_by_nick(
            &self,
            request: tonic::Request<super::GetUserInfoByNickRequest>,
        ) -> Result<tonic::Response<super::GetUserInfoByNickResponse>, tonic::Status>;
        #[doc = " Returns on-chain user info by phone number if user exists"]
        async fn get_user_info_by_number(
            &self,
            request: tonic::Request<super::GetUserInfoByNumberRequest>,
        ) -> Result<tonic::Response<super::GetUserInfoByNumberResponse>, tonic::Status>;
        #[doc = " Returns on-chain user info by account id if user exists"]
        async fn get_user_info_by_account(
            &self,
            request: tonic::Request<super::GetUserInfoByAccountRequest>,
        ) -> Result<tonic::Response<super::GetUserInfoByAccountResponse>, tonic::Status>;
        #[doc = " Returns the identity of all phone verifiers registered on chain"]
        async fn get_phone_verifiers(
            &self,
            request: tonic::Request<super::GetPhoneVerifiersRequest>,
        ) -> Result<tonic::Response<super::GetPhoneVerifiersResponse>, tonic::Status>;
        #[doc = " Returns all char traits on-chain"]
        async fn get_char_traits(
            &self,
            request: tonic::Request<super::GetCharTraitsRequest>,
        ) -> Result<tonic::Response<super::GetCharTraitsResponse>, tonic::Status>;
        #[doc = " Returns the current blockchain state"]
        async fn get_blockchain_data(
            &self,
            request: tonic::Request<super::GetBlockchainDataRequest>,
        ) -> Result<tonic::Response<super::GetBlockchainDataResponse>, tonic::Status>;
        #[doc = " Returns the current blockchain state"]
        async fn get_genesis_data(
            &self,
            request: tonic::Request<super::GetGenesisDataRequest>,
        ) -> Result<tonic::Response<super::GetGenesisDataResponse>, tonic::Status>;
        #[doc = " Submit a signed transaction to the blockchain"]
        async fn submit_transaction(
            &self,
            request: tonic::Request<super::SubmitTransactionRequest>,
        ) -> Result<tonic::Response<super::SubmitTransactionResponse>, tonic::Status>;
        #[doc = " Get all transactions between two account, included transactions in the pool and not yet on-chain"]
        #[doc = " Results include txs current status"]
        async fn get_transactions(
            &self,
            request: tonic::Request<super::GetTransactionsRequest>,
        ) -> Result<tonic::Response<super::GetTransactionsResponse>, tonic::Status>;
        #[doc = " Get transaction data by its digest hash. Transaction may be in pool or on-chain"]
        async fn get_transaction(
            &self,
            request: tonic::Request<super::GetTransactionRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status>;
        #[doc = " Get execution events for one or more blocks"]
        async fn get_blockchain_events(
            &self,
            request: tonic::Request<super::GetBlockchainEventsRequest>,
        ) -> Result<tonic::Response<super::GetBlockchainEventsResponse>, tonic::Status>;
    }
    #[doc = " Unified public API provided by blockchain nodes and verifiers"]
    #[derive(Debug)]
    pub struct ApiServiceServer<T: ApiService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ApiService> ApiServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ApiServiceServer<T>
    where
        T: ApiService,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/karma_coin.api.ApiService/GetUserInfoByNick" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserInfoByNickSvc<T: ApiService>(pub Arc<T>);
                    impl<T: ApiService> tonic::server::UnaryService<super::GetUserInfoByNickRequest>
                        for GetUserInfoByNickSvc<T>
                    {
                        type Response = super::GetUserInfoByNickResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserInfoByNickRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_user_info_by_nick(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserInfoByNickSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
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
                    impl<T: ApiService>
                        tonic::server::UnaryService<super::GetUserInfoByNumberRequest>
                        for GetUserInfoByNumberSvc<T>
                    {
                        type Response = super::GetUserInfoByNumberResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserInfoByNumberRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_user_info_by_number(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
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
                    impl<T: ApiService>
                        tonic::server::UnaryService<super::GetUserInfoByAccountRequest>
                        for GetUserInfoByAccountSvc<T>
                    {
                        type Response = super::GetUserInfoByAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserInfoByAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_user_info_by_account(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetPhoneVerifiers" => {
                    #[allow(non_camel_case_types)]
                    struct GetPhoneVerifiersSvc<T: ApiService>(pub Arc<T>);
                    impl<T: ApiService> tonic::server::UnaryService<super::GetPhoneVerifiersRequest>
                        for GetPhoneVerifiersSvc<T>
                    {
                        type Response = super::GetPhoneVerifiersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPhoneVerifiersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_phone_verifiers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPhoneVerifiersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.api.ApiService/GetCharTraits" => {
                    #[allow(non_camel_case_types)]
                    struct GetCharTraitsSvc<T: ApiService>(pub Arc<T>);
                    impl<T: ApiService> tonic::server::UnaryService<super::GetCharTraitsRequest>
                        for GetCharTraitsSvc<T>
                    {
                        type Response = super::GetCharTraitsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCharTraitsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_char_traits(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCharTraitsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
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
                    impl<T: ApiService> tonic::server::UnaryService<super::GetBlockchainDataRequest>
                        for GetBlockchainDataSvc<T>
                    {
                        type Response = super::GetBlockchainDataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockchainDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_blockchain_data(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
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
                    impl<T: ApiService> tonic::server::UnaryService<super::GetGenesisDataRequest>
                        for GetGenesisDataSvc<T>
                    {
                        type Response = super::GetGenesisDataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGenesisDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_genesis_data(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
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
                    impl<T: ApiService> tonic::server::UnaryService<super::SubmitTransactionRequest>
                        for SubmitTransactionSvc<T>
                    {
                        type Response = super::SubmitTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).submit_transaction(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
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
                    impl<T: ApiService> tonic::server::UnaryService<super::GetTransactionsRequest>
                        for GetTransactionsSvc<T>
                    {
                        type Response = super::GetTransactionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transactions(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
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
                    impl<T: ApiService> tonic::server::UnaryService<super::GetTransactionRequest>
                        for GetTransactionSvc<T>
                    {
                        type Response = super::GetTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transaction(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
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
                    impl<T: ApiService>
                        tonic::server::UnaryService<super::GetBlockchainEventsRequest>
                        for GetBlockchainEventsSvc<T>
                    {
                        type Response = super::GetBlockchainEventsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockchainEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_blockchain_events(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
    impl<T: ApiService> tonic::transport::NamedService for ApiServiceServer<T> {
        const NAME: &'static str = "karma_coin.api.ApiService";
    }
}
