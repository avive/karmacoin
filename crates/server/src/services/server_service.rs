// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::api::api_service::ApiService;
use crate::services::db_config_service::BlockchainConfigService;
use crate::services::verifier::verifier_service::VerifierService;
use anyhow::Result;
use base::genesis_config_service::{GenesisConfigService, GetGenesisData};
use base::karma_coin::karma_coin_api::api_service_server::ApiServiceServer;
use base::karma_coin::karma_coin_verifier::verifier_service_server::VerifierServiceServer;
use base::server_config_service::{
    ServerConfigService, GRPC_SERVER_HOST_CONFIG_KEY, GRPC_SERVER_HOST_PORT_CONFIG_KEY,
    SERVER_NAME_CONFIG_KEY,
};
use base::server_config_service::{SetConfigFile, START_VERIFIER_SERVICE_CONFIG_KEY};
use db::db_service::{DatabaseService, Destroy};
use tonic::transport::*;

use crate::services::blockchain::karma_rewards_service::KarmaRewardsService;

//use crate::services::blockchain::karma_rewards_service::{
//    KarmaRewardsService, ProcessKarmaRewards,
//};

use crate::services::blockchain::backup_chain_service::BackupChainService;
use base::karma_coin::karma_coin_api::GetGenesisDataRequest;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
use xactor::*;

/// ServerService is a full node p2p network server
/// todo: ServerService should maintain node id identity (for protocol purposes)
#[derive(Default)]
pub struct ServerService {}

#[async_trait::async_trait]
impl Actor for ServerService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // start the config services to config db, blockchain and the server
        let genesis_config_service = GenesisConfigService::from_registry().await?;
        let server_config_service = ServerConfigService::from_registry().await?;

        /*
        // generate some keys
        let key1 = KeyPair::new();
        info!(
            "key1 private: {}",
            hex_string(key1.private_key.unwrap().key.as_ref())
        );
        info!(
            "key1 public: {}",
            hex_string(key1.public_key.unwrap().key.as_ref())
        );

        let key2 = KeyPair::new();
        info!(
            "key2 private: {}",
            hex_string(key2.private_key.unwrap().key.as_ref())
        );
        info!(
            "key2 public: {}",
            hex_string(key2.public_key.unwrap().key.as_ref())
        );*/

        server_config_service
            .call(SetConfigFile {
                config_file: "./config.yaml".to_string(),
            })
            .await??;

        let genesis_data = genesis_config_service
            .call(GetGenesisData {
                request: GetGenesisDataRequest {},
            })
            .await??
            .genesis_data
            .unwrap();

        info!("genesis data: {}", genesis_data);

        BlockchainConfigService::from_registry().await?;

        // if we start a verifier then load private secrets from an external verifier config file
        if ServerConfigService::get_bool(START_VERIFIER_SERVICE_CONFIG_KEY.into())
            .await?
            .unwrap()
        {
            VerifierService::from_registry().await?;
        }

        info!("starting karma rewards service...");

        // start the karma rewards service
        KarmaRewardsService::from_registry().await?;

        // start the backup chain service
        BackupChainService::from_registry().await?;

        info!("started");
        Ok(())
    }
}

impl Service for ServerService {}

//////////////////////////

/// Close the db and delete it - used in tests
#[message(result = "Result<()>")]
pub struct DestroyDb;

/// Destroy db
#[async_trait::async_trait]
impl Handler<DestroyDb> for ServerService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: DestroyDb) -> Result<()> {
        let db = DatabaseService::from_registry().await.unwrap();
        let _ = db.call(Destroy).await?.unwrap();
        Ok(())
    }
}

///////////////////////////

#[message(result = "Result<()>")]
pub struct Startup;

/// Start the grpc server
#[async_trait::async_trait]
impl Handler<Startup> for ServerService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Startup) -> Result<()> {
        info!("configuring server...");

        let server_name = ServerConfigService::get(SERVER_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();

        let host = ServerConfigService::get(GRPC_SERVER_HOST_CONFIG_KEY.into())
            .await?
            .unwrap();

        let port = ServerConfigService::get_u64(GRPC_SERVER_HOST_PORT_CONFIG_KEY.into())
            .await?
            .unwrap() as u32;

        self.start_grpc_server(port, host, server_name).await?;

        info!("KC blockchain grpc server started");

        Ok(())
    }
}

impl ServerService {
    /// Starts the server's grpc services
    async fn start_grpc_server(&self, port: u32, host: String, peer_name: String) -> Result<()> {
        // setup grpc server and services
        let grpc_server_addr = format!("{}:{}", host, port).parse()?;
        info!(
            "starting {} grpc server on: {}",
            peer_name, grpc_server_addr
        );

        let start_verifier =
            ServerConfigService::get_bool(START_VERIFIER_SERVICE_CONFIG_KEY.into())
                .await?
                .unwrap();

        let (mut api_health_reporter, api_health_service) = tonic_health::server::health_reporter();
        api_health_reporter
            .set_serving::<ApiServiceServer<ApiService>>()
            .await;

        let reflection_server = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(base::GRPC_DESCRIPTOR)
            .build()?;

        spawn(async move {
            // this only return when server is stopped due to error or shutdown
            let mut router = Server::builder()
                .accept_http1(true)
                //.tls_config(tls).unwrap()
                .layer(CorsLayer::very_permissive())
                .layer(GrpcWebLayer::new())
                .add_service(reflection_server)
                .add_service(api_health_service)
                .add_service(ApiServiceServer::new(ApiService::default()));

            if start_verifier {
                router = router.add_service(VerifierServiceServer::new(VerifierService::default()));
            }

            let res = router.serve(grpc_server_addr).await;

            if res.is_err() {
                info!("grpc server stopped due to error: {:?}", res.err().unwrap());
            } else {
                info!("grpc server stopped");
            }
        });

        Ok(())
    }
}
