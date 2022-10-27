// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
extern crate base;
extern crate clap;
extern crate db;

use base::logging_service::{InitLogger, LoggingService};
use base::server_config_service::{ServerConfigService, SetConfigFile, PEER_NAME_CONFIG_KEY};
use server::server_service::{ServerService, Startup};
use tokio::signal;

use clap::{App, Arg};
use db::db_service::DatabaseService;

use xactor::*;

// Start a client app - good for testability / integration testing
pub async fn start() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Subnet Playground")
        .version("0.1.0")
        .author("Foo Bar. <foo@bar.goo>")
        .about("Does awesome things")
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

    let config = ServerConfigService::from_registry().await.unwrap();

    // merge values from config file over default config it it is provided via flag
    if let Some(conf_file) = matches.value_of("config") {
        config
            .call(SetConfigFile {
                config_file: conf_file.into(),
            })
            .await?
            .unwrap();
    }

    let peer_name = ServerConfigService::get(PEER_NAME_CONFIG_KEY.into())
        .await?
        .unwrap();

    // Start app logger
    let logging = LoggingService::from_registry().await.unwrap();
    let _ = logging
        .call(InitLogger {
            peer_name: peer_name.clone(),
            brief: true, // todo: take from config
        })
        .await
        .unwrap();

    // Start network server
    let server = ServerService::from_registry().await.unwrap();

    server.call(Startup {}).await??;

    // test logging
    debug!("Services started");

    signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c signal");

    debug!("stopping server-app via ctrl-c signal...");
    tokio::task::spawn(async {
        // stop the db so it has a chance to destroy itself if it is configured to destroy storage on stop...
        let mut db_service = DatabaseService::from_registry().await.unwrap();
        let _ = db_service.stop(None);
        debug!("resources cleanup completed");
    })
    .await
    .unwrap();

    Ok(())
}
