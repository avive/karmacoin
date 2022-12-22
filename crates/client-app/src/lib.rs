// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
extern crate base;
extern crate clap;
extern crate db;

use base::client_config_service::{ClientConfigService, SetConfigFile, CLIENT_NAME_CONFIG_KEY};
use base::logging_service::{InitLogger, LoggingService};
use clap::{App, Arg};
use client::client::{Client, StartGrpcServer};
use db::db_service::DatabaseService;
use tokio::signal;
use base::server_config_service::{GRPC_SERVER_HOST_CONFIG_KEY, GRPC_SERVER_HOST_PORT_CONFIG_KEY};
use xactor::*;

// Start a client app - good for testability / integration testing
pub async fn start() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("KarmaCoin Simple Client")
        .version("0.1.0")
        .author("ae <a@karmaco.in>")
        .about("Spreads good karma")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config = ClientConfigService::from_registry().await.unwrap();

    // merge values from config file over default config it it is provided via flag
    if let Some(conf_file) = matches.value_of("config") {
        config
            .call(SetConfigFile {
                config_file: conf_file.into(),
            })
            .await?
            .unwrap();
    }

    let client_name = ClientConfigService::get(CLIENT_NAME_CONFIG_KEY.into())
        .await?
        .unwrap();
    let grpc_host = ClientConfigService::get(GRPC_SERVER_HOST_CONFIG_KEY.into())
        .await?
        .unwrap();
    let grpc_port = ClientConfigService::get_u64(GRPC_SERVER_HOST_PORT_CONFIG_KEY.into())
        .await?
        .unwrap();

    // init base services
    info!("client-app starting...");

    // Start app logger
    let logging = LoggingService::from_registry().await?;
    let _ = logging
        .call(InitLogger {
            peer_name: client_name.clone(),
            brief: true, // todo: take from config
        })
        .await?;

    // test logging

    let client = Client::from_registry().await.unwrap();
    let _ = client
        .call(StartGrpcServer {
            grpc_port: grpc_port as u32,
            grpc_host,
            client_name: client_name.to_string(),
        })
        .await
        .unwrap();

    info!("client services started");

    signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c signal");

    debug!("stopping client-app via ctrl-c signal...");

    tokio::task::spawn(async {
        // stop the db service so it has a chance to destroy itself if it is configured to destroy storage on stop...
        let mut db_service = DatabaseService::from_registry().await.unwrap();
        let _ = db_service.stop(None);
        debug!("resources cleanup completed");
    })
    .await
    .unwrap();

    Ok(())
}
