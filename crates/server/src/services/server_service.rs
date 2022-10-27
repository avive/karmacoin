//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//


use anyhow::Result;
//use base::
use rocksdb::{ColumnFamilyDescriptor, Options};

use crate::services::api_service::ApiService;
use base::karma_coin::karma_coin_api::api_service_server::ApiServiceServer;

use base::server_config_service::{
    ServerConfigService, DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY, GRPC_HOST_CONFIG_KEY,
    GRPC_SERVER_PORT_CONFIG_KEY, PEER_NAME_CONFIG_KEY,
};
use db::db_service::{
    DatabaseService, Destroy, PROVIDER_COL_FAMILY, PROVIDER_DISTRIBUTED_DATA_COL_FAMILY,
    PROVIDER_USER_DATA_COL_FAMILY, TESTS_COL_FAMILY,
};
use tonic::transport::Server;
use xactor::*;

pub const SNP_PROTOCOL_VERSION: &str = "0.1.0";

/// ServerService is a full node p2p network server
/// todo: ServerService should maintain node id identity (for protocol purposes)
#[derive(Default)]
pub struct ServerService {}

#[async_trait::async_trait]
impl Actor for ServerService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // Init the Upsetter db
        let _db = DatabaseService::from_registry().await.unwrap();

        info!("ServerService started");
        Ok(())
    }
}

impl Service for ServerService {}

//////////////////////////

/// Close the db and delete it (for testing purposes)
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
        info!("configuring db...");

        let peer_name = ServerConfigService::get(PEER_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();
        let host = ServerConfigService::get(GRPC_HOST_CONFIG_KEY.into())
            .await?
            .unwrap();
        let port = ServerConfigService::get_u64(GRPC_SERVER_PORT_CONFIG_KEY.into())
            .await?
            .unwrap() as u32;

        let db_name = ServerConfigService::get(DB_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();

        let drop_on_exit = ServerConfigService::get_bool(DROP_DB_CONFIG_KEY.into())
            .await?
            .unwrap();

        DatabaseService::config_db(db::db_service::Configure {
            drop_on_exit,
            db_name,
            col_descriptors: vec![
                ColumnFamilyDescriptor::new(PROVIDER_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(PROVIDER_USER_DATA_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(
                    PROVIDER_DISTRIBUTED_DATA_COL_FAMILY,
                    Options::default(),
                ),
                ColumnFamilyDescriptor::new(TESTS_COL_FAMILY, Options::default()),
            ],
        })
        .await?;

        self.start_grpc_server(port, host, peer_name).await?;

        info!("services started");

        Ok(())
    }
}

impl ServerService {
    async fn start_grpc_server(&self, port: u32, host: String, peer_name: String) -> Result<()> {
        // setup grpc server and services
        let grpc_server_addr = format!("{}:{}", host, port).parse().unwrap();
        info!(
            "starting {} grpc server on: {}",
            peer_name, grpc_server_addr
        );

        // start health service and indicate we are serving MyMessagingService
        // let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        // health_reporter
        //    .set_serving::<ProviderCoreServiceServer<ServerMessagingService>>()
        //    .await;

        // todo: server admin service should be deployed on different server with different port for security and only be available inside provider network or only locally.

        tokio::task::spawn(async move {
            // all services that should be started must be added below
            let res = Server::builder()
                //.add_service(ProviderCoreServiceServer::new(messaging_service))
                .add_service(ApiServiceServer::new(ApiService::default()))
                //.add_service(health_service)
                .serve(grpc_server_addr)
                .await;

            if res.is_err() {
                info!("grpc server stopped due to: {:?}", res.err().unwrap());
            } else {
                info!("grpc server stopped");
            }
        });

        Ok(())
    }
}
