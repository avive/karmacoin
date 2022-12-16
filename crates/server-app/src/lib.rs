// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
extern crate base;
extern crate clap;
extern crate db;

use base::logging_service::{InitLogger, LoggingService};
use base::server_config_service::{SERVER_NAME_CONFIG_KEY, ServerConfigService, SetConfigFile};
use server::server_service::{ServerService, Startup};
use tokio::signal;

use clap::{App, Arg};
use db::db_service::DatabaseService;

use xactor::*;

// Start a client app - good for testability / integration testing
pub async fn start() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("KarmaCoin Server")
        .version("0.1.0")
        .author("AE  <a@karmaco.in>")
        .about("The coin for all of us")
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

    let server_name = ServerConfigService::get(SERVER_NAME_CONFIG_KEY.into())
        .await?
        .unwrap();

    // Start app logger
    let logging = LoggingService::from_registry().await.unwrap();
    let _ = logging
        .call(InitLogger {
            peer_name: server_name.clone(),
            brief: false, // todo: take from config
        })
        .await
        .unwrap();

    // Start network server
    let server = ServerService::from_registry().await.unwrap();

    server.call(Startup {}).await??;

    // test logging
    info!("Services started");

    signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c signal");

    debug!("stopping server-app via ctrl-c signal...");
    spawn(async {
        // stop the db so it has a chance to destroy itself if it is configured to destroy storage on stop...
        let mut db_service = DatabaseService::from_registry().await.unwrap();
        let _ = db_service.stop(None);
        debug!("resources cleanup completed");
    })
    .await
    .unwrap();

    Ok(())
}
