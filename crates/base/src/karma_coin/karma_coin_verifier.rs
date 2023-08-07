#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendVerificationCodeRequest {
    #[prost(string, tag = "1")]
    pub mobile_number: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendVerificationCodeResponse {
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
}
/// Verier Info is used to return the network the id and dial-up info of active verifiers
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifierInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub account_id: ::core::option::Option<super::core_types::AccountId>,
    /// ip:port
    #[prost(string, tag = "3")]
    pub verifier_endpoint_ip4: ::prost::alloc::string::String,
    /// ip:port
    #[prost(string, tag = "4")]
    pub verifier_endpoint_ip6: ::prost::alloc::string::String,
    /// ip:port
    #[prost(string, tag = "5")]
    pub api_endpoint_ip4: ::prost::alloc::string::String,
    /// ip:port
    #[prost(string, tag = "6")]
    pub api_endpoint_ip6: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub signature: ::core::option::Option<super::core_types::Signature>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNumberRequest {
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    #[prost(message, optional, tag = "2")]
    pub account_id: ::core::option::Option<super::core_types::AccountId>,
    #[prost(message, optional, tag = "3")]
    pub mobile_number: ::core::option::Option<super::core_types::MobileNumber>,
    #[prost(string, tag = "4")]
    pub requested_user_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub signature: ::core::option::Option<super::core_types::Signature>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNumberResponse {
    #[prost(message, optional, tag = "1")]
    pub user_verification_data: ::core::option::Option<
        super::core_types::UserVerificationData,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNumberRequestDataEx {
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    #[prost(message, optional, tag = "2")]
    pub account_id: ::core::option::Option<super::core_types::AccountId>,
    #[prost(message, optional, tag = "3")]
    pub mobile_number: ::core::option::Option<super::core_types::MobileNumber>,
    #[prost(string, tag = "4")]
    pub requested_user_name: ::prost::alloc::string::String,
    /// optional token to bypass verification
    #[prost(bytes = "vec", tag = "5")]
    pub bypass_token: ::prost::alloc::vec::Vec<u8>,
    /// Twilio whatsapp verification code
    #[prost(string, tag = "6")]
    pub verification_code: ::prost::alloc::string::String,
    /// Twilio verification sid (obtained when verify was called from client in response)
    #[prost(string, tag = "7")]
    pub verification_sid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNumberRequestEx {
    /// serialized VerifyNumberRequestDataEx
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// User signature of binary data field 1
    /// Public key is account_id in the data
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// / Data object stored in db to track invite sms messages
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmsInviteMetadata {
    /// invited person mobile phone number
    #[prost(message, optional, tag = "1")]
    pub mobile_number: ::core::option::Option<super::core_types::MobileNumber>,
    /// the time of the last invite sms message sent
    #[prost(uint64, tag = "2")]
    pub last_message_sent_time_stamp: u64,
    /// total number of invite sms messages sent
    #[prost(uint32, tag = "3")]
    pub messages_sent: u32,
    /// inviter mobile phone number (from appreciation tx)
    #[prost(message, optional, tag = "4")]
    pub inviter_account_id: ::core::option::Option<super::core_types::AccountId>,
    /// the hash of the payment tx that triggers this invite
    #[prost(bytes = "vec", tag = "5")]
    pub invite_tx_hash: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod verifier_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// mobile phone numbers verifier api service
    #[derive(Debug, Clone)]
    pub struct VerifierServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VerifierServiceClient<tonic::transport::Channel> {
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
    impl<T> VerifierServiceClient<T>
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
        ) -> VerifierServiceClient<InterceptedService<T, F>>
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
            VerifierServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Request to verify a number by providing code sent via sms from verifier
        /// note that VerifyNumberResponse was lifted to types as it is used in signup transactions
        pub async fn verify_number(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyNumberRequest>,
        ) -> Result<tonic::Response<super::VerifyNumberResponse>, tonic::Status> {
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
                "/karma_coin.verifier.VerifierService/VerifyNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Extended api - verifies number via Twilio whatsapp given user code
        pub async fn verify_number_ex(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyNumberRequestEx>,
        ) -> Result<tonic::Response<super::VerifyNumberResponse>, tonic::Status> {
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
                "/karma_coin.verifier.VerifierService/VerifyNumberEx",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Send verification code to the user's mobile number via whatsapp
        pub async fn send_verification_code(
            &mut self,
            request: impl tonic::IntoRequest<super::SendVerificationCodeRequest>,
        ) -> Result<
            tonic::Response<super::SendVerificationCodeResponse>,
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
                "/karma_coin.verifier.VerifierService/SendVerificationCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod verifier_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VerifierServiceServer.
    #[async_trait]
    pub trait VerifierService: Send + Sync + 'static {
        /// Request to verify a number by providing code sent via sms from verifier
        /// note that VerifyNumberResponse was lifted to types as it is used in signup transactions
        async fn verify_number(
            &self,
            request: tonic::Request<super::VerifyNumberRequest>,
        ) -> Result<tonic::Response<super::VerifyNumberResponse>, tonic::Status>;
        /// Extended api - verifies number via Twilio whatsapp given user code
        async fn verify_number_ex(
            &self,
            request: tonic::Request<super::VerifyNumberRequestEx>,
        ) -> Result<tonic::Response<super::VerifyNumberResponse>, tonic::Status>;
        /// Send verification code to the user's mobile number via whatsapp
        async fn send_verification_code(
            &self,
            request: tonic::Request<super::SendVerificationCodeRequest>,
        ) -> Result<tonic::Response<super::SendVerificationCodeResponse>, tonic::Status>;
    }
    /// mobile phone numbers verifier api service
    #[derive(Debug)]
    pub struct VerifierServiceServer<T: VerifierService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: VerifierService> VerifierServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VerifierServiceServer<T>
    where
        T: VerifierService,
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
                "/karma_coin.verifier.VerifierService/VerifyNumber" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyNumberSvc<T: VerifierService>(pub Arc<T>);
                    impl<
                        T: VerifierService,
                    > tonic::server::UnaryService<super::VerifyNumberRequest>
                    for VerifyNumberSvc<T> {
                        type Response = super::VerifyNumberResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyNumberRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).verify_number(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VerifyNumberSvc(inner);
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
                "/karma_coin.verifier.VerifierService/VerifyNumberEx" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyNumberExSvc<T: VerifierService>(pub Arc<T>);
                    impl<
                        T: VerifierService,
                    > tonic::server::UnaryService<super::VerifyNumberRequestEx>
                    for VerifyNumberExSvc<T> {
                        type Response = super::VerifyNumberResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyNumberRequestEx>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).verify_number_ex(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VerifyNumberExSvc(inner);
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
                "/karma_coin.verifier.VerifierService/SendVerificationCode" => {
                    #[allow(non_camel_case_types)]
                    struct SendVerificationCodeSvc<T: VerifierService>(pub Arc<T>);
                    impl<
                        T: VerifierService,
                    > tonic::server::UnaryService<super::SendVerificationCodeRequest>
                    for SendVerificationCodeSvc<T> {
                        type Response = super::SendVerificationCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendVerificationCodeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_verification_code(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendVerificationCodeSvc(inner);
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
    impl<T: VerifierService> Clone for VerifierServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: VerifierService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VerifierService> tonic::server::NamedService for VerifierServiceServer<T> {
        const NAME: &'static str = "karma_coin.verifier.VerifierService";
    }
}
