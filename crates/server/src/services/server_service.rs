// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::api::api_service::ApiService;
use crate::services::db_config_service::DbConfigService;
use crate::services::verifier::verifier_service::VerifierService;
use anyhow::Result;
use base::genesis_config_service::GenesisConfigService;
use base::karma_coin::karma_coin_api::api_service_server::ApiServiceServer;
use base::karma_coin::karma_coin_verifier::verifier_service_server::VerifierServiceServer;
use base::server_config_service::{
    ServerConfigService, GRPC_SERVER_HOST_CONFIG_KEY, GRPC_SERVER_HOST_PORT_CONFIG_KEY,
    SERVER_NAME_CONFIG_KEY,
};
use db::db_service::{DatabaseService, Destroy};
use tonic::transport::Server;
use xactor::*;

/// ServerService is a full node p2p network server
/// todo: ServerService should maintain node id identity (for protocol purposes)
#[derive(Default)]
pub struct ServerService {}

#[async_trait::async_trait]
impl Actor for ServerService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // start the config services to config db, blockchain and the server
        DbConfigService::from_registry().await?;
        GenesisConfigService::from_registry().await?;
        ServerConfigService::from_registry().await?;

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
pub struct Startup {}

/// Start the grpc server
#[async_trait::async_trait]
impl Handler<Startup> for ServerService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Startup) -> Result<()> {
        info!("configuring KarmaCoin server...");

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

        info!("GRPC server started");

        Ok(())
    }
}

impl ServerService {
    /// Starts the server's grpc services
    async fn start_grpc_server(&self, port: u32, host: String, peer_name: String) -> Result<()> {
        // setup grpc server and services
        let grpc_server_addr = format!("{}:{}", host, port).parse().unwrap();
        info!(
            "starting {} grpc server on: {}",
            peer_name, grpc_server_addr
        );

        let (mut api_health_reporter, api_health_service) = tonic_health::server::health_reporter();
        api_health_reporter
            .set_serving::<ApiServiceServer<ApiService>>()
            .await;

        let (mut verifier_health_reporter, verifier_health_service) =
            tonic_health::server::health_reporter();
        verifier_health_reporter
            .set_serving::<VerifierServiceServer<VerifierService>>()
            .await;

        spawn(async move {
            // all services that should be started must be added below
            let res = Server::builder()
                .add_service(ApiServiceServer::new(ApiService::default()))
                .add_service(VerifierServiceServer::new(VerifierService::default()))
                .add_service(api_health_service)
                .add_service(verifier_health_service)
                .serve(grpc_server_addr)
                .await;

            if res.is_err() {
                info!("grpc server stopped due to: {:?}", res.err().unwrap());
            } else {
                info!("GRPC server stopped");
            }
        });

        Ok(())
    }
}
