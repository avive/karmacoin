// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use chrono::prelude::*;
use env_logger::fmt::Color;
use env_logger::Builder;
use log::*;
use std::env;
use std::io::Write;
use xactor::*;

pub struct LoggingService {}

impl Service for LoggingService {}

#[async_trait::async_trait]
impl Actor for LoggingService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("LoggingService started");
        Ok(())
    }
}

#[message(result = "Result<()>")]
pub struct InitLogger {
    pub peer_name: String,
    pub brief: bool,
}

#[async_trait::async_trait]
impl Handler<InitLogger> for LoggingService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: InitLogger) -> Result<()> {
        let mut builder = Builder::new();

        builder
            .format_level(true)
            .format_timestamp(None)
            .format(move |buf, record| {
                let level_style = buf.default_level_style(record.level());
                let mut peer_name_style = buf.style();
                peer_name_style.set_color(Color::Yellow).set_bold(true);
                let mut file_name_style = buf.style();
                file_name_style.set_color(Color::Blue);
                let file_name = format!(
                    "{} {}",
                    record.file().unwrap().split('/').last().unwrap(),
                    record.line().unwrap()
                );

                let now: DateTime<Local> = Local::now();
                let now_disp = format!(
                    "{}.{}.{}:{}",
                    now.hour(),
                    now.minute(),
                    now.second(),
                    now.timestamp_subsec_millis()
                );

                match msg.brief {
                    true => writeln!(
                        buf,
                        "{} {}",
                        peer_name_style.value(msg.peer_name.clone()),
                        record.args(),
                    ),
                    false => writeln!(
                        buf,
                        "{} {}\t {} {} {}",
                        peer_name_style.value(msg.peer_name.clone()),
                        record.args(),
                        level_style.value(record.level()),
                        file_name_style.value(file_name),
                        now_disp,
                    ),
                }
            })
            .filter(None, LevelFilter::Info);

        if env::var("RUST_LOG").is_ok() {
            builder.parse_filters(&env::var("RUST_LOG").unwrap());
        }

        builder.init();
        Ok(())
    }
}

impl Default for LoggingService {
    fn default() -> Self {
        LoggingService {}
    }
}
