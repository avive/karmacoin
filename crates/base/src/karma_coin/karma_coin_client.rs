#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureRequest {
    /// user's nickname
    #[prost(string, tag = "1")]
    pub nickname: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignUpRequest {
    /// verifier grpc endpoint. e.g \\[:1\\]:5438
    #[prost(string, tag = "1")]
    pub verifier_endpoint: ::prost::alloc::string::String,
    /// karmacoin api endpoint. e.g. \\[:1\\]:2351
    #[prost(string, tag = "2")]
    pub api_endpoint: ::prost::alloc::string::String,
    /// verifier account id that should be trusted
    #[prost(message, optional, tag = "3")]
    pub verifier_account_id: ::core::option::Option<super::core_types::AccountId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignUpResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountStateRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePhoneNumberRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePhoneNumberResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserInfoResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendCoinRequest {
    /// receiver's mobile phone number
    #[prost(message, optional, tag = "1")]
    pub mobile_number: ::core::option::Option<super::core_types::MobileNumber>,
    /// amount to send
    #[prost(uint64, tag = "2")]
    pub amount: u64,
    /// transaction fee
    #[prost(uint64, tag = "3")]
    pub fee: u64,
    /// char trait to appreciate
    #[prost(message, optional, tag = "4")]
    pub char_trait: ::core::option::Option<super::core_types::CharTrait>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendCoinResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountStateResponse {
    /// public user info includes balances, karma score, etc...
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::core_types::User>,
    /// all transactions known to client for the user's account
    #[prost(message, repeated, tag = "2")]
    pub transactions: ::prost::alloc::vec::Vec<super::core_types::SignedTransaction>,
}
/// Generated client implementations.
pub mod client_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A simple client API used for instrumenting a client and integration tests
    #[derive(Debug, Clone)]
    pub struct ClientApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ClientApiClient<tonic::transport::Channel> {
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
    impl<T> ClientApiClient<T>
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
        ) -> ClientApiClient<InterceptedService<T, F>>
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
            ClientApiClient::new(InterceptedService::new(inner, interceptor))
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
        /// Configure the client with config data
        pub async fn configure(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigureRequest>,
        ) -> Result<tonic::Response<super::ConfigureResponse>, tonic::Status> {
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
                "/karma_coin.client.ClientApi/Configure",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sign up using mobile number
        pub async fn sign_up(
            &mut self,
            request: impl tonic::IntoRequest<super::SignUpRequest>,
        ) -> Result<tonic::Response<super::SignUpResponse>, tonic::Status> {
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
                "/karma_coin.client.ClientApi/SignUp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update user public info such as nickname or phone number
        pub async fn update_user_info(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserInfoRequest>,
        ) -> Result<tonic::Response<super::UpdateUserInfoResponse>, tonic::Status> {
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
                "/karma_coin.client.ClientApi/UpdateUserInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Send a coin to another user and optionally appreciate
        pub async fn send_coin(
            &mut self,
            request: impl tonic::IntoRequest<super::SendCoinRequest>,
        ) -> Result<tonic::Response<super::SendCoinResponse>, tonic::Status> {
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
                "/karma_coin.client.ClientApi/SendCoin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get current account state such as balance, karma score, char traits and transactions
        pub async fn get_account_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountStateRequest>,
        ) -> Result<tonic::Response<super::GetAccountStateResponse>, tonic::Status> {
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
                "/karma_coin.client.ClientApi/GetAccountData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod client_api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ClientApiServer.
    #[async_trait]
    pub trait ClientApi: Send + Sync + 'static {
        /// Configure the client with config data
        async fn configure(
            &self,
            request: tonic::Request<super::ConfigureRequest>,
        ) -> Result<tonic::Response<super::ConfigureResponse>, tonic::Status>;
        /// Sign up using mobile number
        async fn sign_up(
            &self,
            request: tonic::Request<super::SignUpRequest>,
        ) -> Result<tonic::Response<super::SignUpResponse>, tonic::Status>;
        /// Update user public info such as nickname or phone number
        async fn update_user_info(
            &self,
            request: tonic::Request<super::UpdateUserInfoRequest>,
        ) -> Result<tonic::Response<super::UpdateUserInfoResponse>, tonic::Status>;
        /// Send a coin to another user and optionally appreciate
        async fn send_coin(
            &self,
            request: tonic::Request<super::SendCoinRequest>,
        ) -> Result<tonic::Response<super::SendCoinResponse>, tonic::Status>;
        /// Get current account state such as balance, karma score, char traits and transactions
        async fn get_account_data(
            &self,
            request: tonic::Request<super::GetAccountStateRequest>,
        ) -> Result<tonic::Response<super::GetAccountStateResponse>, tonic::Status>;
    }
    /// A simple client API used for instrumenting a client and integration tests
    #[derive(Debug)]
    pub struct ClientApiServer<T: ClientApi> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ClientApi> ClientApiServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ClientApiServer<T>
    where
        T: ClientApi,
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
                "/karma_coin.client.ClientApi/Configure" => {
                    #[allow(non_camel_case_types)]
                    struct ConfigureSvc<T: ClientApi>(pub Arc<T>);
                    impl<
                        T: ClientApi,
                    > tonic::server::UnaryService<super::ConfigureRequest>
                    for ConfigureSvc<T> {
                        type Response = super::ConfigureResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigureRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).configure(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConfigureSvc(inner);
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
                "/karma_coin.client.ClientApi/SignUp" => {
                    #[allow(non_camel_case_types)]
                    struct SignUpSvc<T: ClientApi>(pub Arc<T>);
                    impl<T: ClientApi> tonic::server::UnaryService<super::SignUpRequest>
                    for SignUpSvc<T> {
                        type Response = super::SignUpResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignUpRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sign_up(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignUpSvc(inner);
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
                "/karma_coin.client.ClientApi/UpdateUserInfo" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserInfoSvc<T: ClientApi>(pub Arc<T>);
                    impl<
                        T: ClientApi,
                    > tonic::server::UnaryService<super::UpdateUserInfoRequest>
                    for UpdateUserInfoSvc<T> {
                        type Response = super::UpdateUserInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_user_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateUserInfoSvc(inner);
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
                "/karma_coin.client.ClientApi/SendCoin" => {
                    #[allow(non_camel_case_types)]
                    struct SendCoinSvc<T: ClientApi>(pub Arc<T>);
                    impl<
                        T: ClientApi,
                    > tonic::server::UnaryService<super::SendCoinRequest>
                    for SendCoinSvc<T> {
                        type Response = super::SendCoinResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendCoinRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_coin(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendCoinSvc(inner);
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
                "/karma_coin.client.ClientApi/GetAccountData" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountDataSvc<T: ClientApi>(pub Arc<T>);
                    impl<
                        T: ClientApi,
                    > tonic::server::UnaryService<super::GetAccountStateRequest>
                    for GetAccountDataSvc<T> {
                        type Response = super::GetAccountStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_account_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountDataSvc(inner);
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
    impl<T: ClientApi> Clone for ClientApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ClientApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ClientApi> tonic::server::NamedService for ClientApiServer<T> {
        const NAME: &'static str = "karma_coin.client.ClientApi";
    }
}
