// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use xactor::*;

mod basic;
mod broker;
mod ping;
mod service;
mod stream;
mod subscriber;
mod supervise;

use basic::basic;
use broker::broker;
use ping::ping;
use service::service;
use stream::stream;
use subscriber::subscriber;
use supervise::supervise;

#[xactor::main]
async fn main() -> Result<()> {
    basic().await?;
    ping().await?;
    service().await?;
    supervise().await?;
    broker().await?;
    stream().await?;
    subscriber().await?;
    Ok(())
}
