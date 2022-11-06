#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterNumberRequest {
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<super::core_types::AccountId>,
    #[prost(message, optional, tag = "2")]
    pub mobile_number: ::core::option::Option<super::core_types::MobileNumber>,
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<super::core_types::Signature>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterNumberResponse {
    #[prost(enumeration = "RegisterNumberResult", tag = "1")]
    pub result: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNumberRequest {
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<super::core_types::AccountId>,
    #[prost(message, optional, tag = "2")]
    pub mobile_number: ::core::option::Option<super::core_types::MobileNumber>,
    /// auth code received via sms
    #[prost(string, tag = "3")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub nickname: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub signature: ::core::option::Option<super::core_types::Signature>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RegisterNumberResult {
    InvalidNumber = 0,
    InvalidSignature = 1,
    PhoneAlreadyRegistered = 2,
    PhoneAccountExists = 3,
    CodeSent = 4,
}
#[doc = r" Generated client implementations."]
pub mod phone_numbers_verifier_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " mobile phone numbers verifier api service"]
    #[derive(Debug, Clone)]
    pub struct PhoneNumbersVerifierServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PhoneNumbersVerifierServiceClient<tonic::transport::Channel> {
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
    impl<T> PhoneNumbersVerifierServiceClient<T>
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
        ) -> PhoneNumbersVerifierServiceClient<InterceptedService<T, F>>
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
            PhoneNumbersVerifierServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Request to register a phone number. Will trigger an SMS to that number"]
        pub async fn register_number(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterNumberRequest>,
        ) -> Result<tonic::Response<super::RegisterNumberResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.verifier.PhoneNumbersVerifierService/RegisterNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Request to verify a number by providing code sent via sms from verifier"]
        pub async fn verify_number(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyNumberRequest>,
        ) -> Result<tonic::Response<super::super::core_types::VerifyNumberResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.verifier.PhoneNumbersVerifierService/VerifyNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod phone_numbers_verifier_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PhoneNumbersVerifierServiceServer."]
    #[async_trait]
    pub trait PhoneNumbersVerifierService: Send + Sync + 'static {
        #[doc = " Request to register a phone number. Will trigger an SMS to that number"]
        async fn register_number(
            &self,
            request: tonic::Request<super::RegisterNumberRequest>,
        ) -> Result<tonic::Response<super::RegisterNumberResponse>, tonic::Status>;
        #[doc = " Request to verify a number by providing code sent via sms from verifier"]
        async fn verify_number(
            &self,
            request: tonic::Request<super::VerifyNumberRequest>,
        ) -> Result<tonic::Response<super::super::core_types::VerifyNumberResponse>, tonic::Status>;
    }
    #[doc = " mobile phone numbers verifier api service"]
    #[derive(Debug)]
    pub struct PhoneNumbersVerifierServiceServer<T: PhoneNumbersVerifierService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PhoneNumbersVerifierService> PhoneNumbersVerifierServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PhoneNumbersVerifierServiceServer<T>
    where
        T: PhoneNumbersVerifierService,
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
                "/karma_coin.verifier.PhoneNumbersVerifierService/RegisterNumber" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterNumberSvc<T: PhoneNumbersVerifierService>(pub Arc<T>);
                    impl<T: PhoneNumbersVerifierService>
                        tonic::server::UnaryService<super::RegisterNumberRequest>
                        for RegisterNumberSvc<T>
                    {
                        type Response = super::RegisterNumberResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterNumberRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).register_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterNumberSvc(inner);
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
                "/karma_coin.verifier.PhoneNumbersVerifierService/VerifyNumber" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyNumberSvc<T: PhoneNumbersVerifierService>(pub Arc<T>);
                    impl<T: PhoneNumbersVerifierService>
                        tonic::server::UnaryService<super::VerifyNumberRequest>
                        for VerifyNumberSvc<T>
                    {
                        type Response = super::super::core_types::VerifyNumberResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyNumberRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).verify_number(request).await };
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
    impl<T: PhoneNumbersVerifierService> Clone for PhoneNumbersVerifierServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PhoneNumbersVerifierService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PhoneNumbersVerifierService> tonic::transport::NamedService
        for PhoneNumbersVerifierServiceServer<T>
    {
        const NAME: &'static str = "karma_coin.verifier.PhoneNumbersVerifierService";
    }
}
