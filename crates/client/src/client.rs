// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::grpc_service::ClientGrpcService;
use anyhow::Result;
use base::client_config_service::ClientConfigService;
use base::client_config_service::TESTS_COL_FAMILY;
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_client::client_api_server::ClientApiServer;
use base::server_config_service::{DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY};
use db::db_service::{Configure, DatabaseService};
use ed25519_dalek::Keypair;
use rand_core::OsRng;
use rocksdb::{ColumnFamilyDescriptor, Options};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
use x25519_dalek::StaticSecret;
use xactor::*;

pub const SNP_PROTOCOL_VERSION: &str = "0.1.0";

/// A simple client creates a new id when it is running
/// and has only one pre_key it uses
/// Currently doesn't use a db or x2dh or dr services for more robust functionality
/// todo: use a DR service to store DR sessions w provider and w other clients instead of hard-coded ones
pub struct Client {
    pub(crate) client_name: String,
    /// client long term ed25519 id
    pub(crate) _client_id: Keypair,
    /// for now we assume only 1 pre-key for the client and we don't create new ones yet
    pub(crate) _pre_key: StaticSecret,
    // A name server client used to communicate with a name service
    // pub(crate) blockchain_service_client: Option<BlockchainServiceClient<Channel>>,
}

impl Default for Client {
    fn default() -> Self {
        let client_id = Keypair::generate(&mut OsRng);
        info!(
            "New client pub id: {}",
            short_hex_string(client_id.public.as_ref())
        );

        Client {
            client_name: "KarmaCoinSimpleClient".into(),
            _client_id: client_id,
            _pre_key: StaticSecret::new(OsRng),
        }
    }
}

impl Service for Client {}

#[async_trait::async_trait]
impl Actor for Client {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // init here system services used by this client

        info!("initializing client db...");
        let db_name = ClientConfigService::get(DB_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();
        let drop_on_exit = ClientConfigService::get_bool(DROP_DB_CONFIG_KEY.into())
            .await?
            .unwrap();

        // no column descriptors ????
        DatabaseService::config_db(Configure {
            drop_on_exit,
            db_name,
            col_descriptors: vec![
                //ColumnFamilyDescriptor::new(PROVIDER_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TESTS_COL_FAMILY, Options::default()),
            ],
        })
        .await?;

        info!("SimpleClient started");
        Ok(())
    }
}

#[message(result = "Result<()>")]
pub struct StartGrpcServer {
    pub grpc_port: u32,
    pub grpc_host: String,
    pub client_name: String,
}

/// Starts this client grpc server
#[async_trait::async_trait]
impl Handler<StartGrpcServer> for Client {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: StartGrpcServer) -> Result<()> {
        // setup grpc server and services
        let grpc_server_addr = format!("{}:{}", msg.grpc_host, msg.grpc_port)
            .parse()
            .unwrap();
        info!(
            "starting {} client grpc server on: {}",
            msg.client_name, grpc_server_addr
        );

        self.client_name = msg.client_name;
        let client_grpc_service = ClientApiServer::new(ClientGrpcService::default());

        spawn(async move {
            let cors = CorsLayer::very_permissive();
            let res = Server::builder()
                .accept_http1(true)
                .layer(cors)
                .layer(GrpcWebLayer::new())
                .add_service(client_grpc_service)
                .serve(grpc_server_addr)
                .await;

            if res.is_err() {
                debug!("client grpc server stopping due to: {:?}", res);
            } else {
                debug!("client grpc Server stopped.");
            }
        });

        Ok(())
    }
}
