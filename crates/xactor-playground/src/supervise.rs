// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use std::time::Duration;
use tokio::time::sleep;
use xactor::*;

#[message]
struct Die;

#[message]
struct Add;

#[message(result = "i32")]
struct Get;

struct MyActor(i32);

impl Actor for MyActor {}

#[async_trait::async_trait]
impl Handler<Add> for MyActor {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _: Add) {
        self.0 += 1;
    }
}

#[async_trait::async_trait]
impl Handler<Get> for MyActor {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _: Get) -> i32 {
        self.0
    }
}

#[async_trait::async_trait]
impl Handler<Die> for MyActor {
    async fn handle(&mut self, ctx: &mut Context<Self>, _: Die) {
        ctx.stop(None);
    }
}

pub async fn supervise() -> Result<()> {
    let addr = Supervisor::start(|| MyActor(0)).await?;

    addr.send(Add)?;
    assert_eq!(addr.call(Get).await?, 1);

    addr.send(Add)?;
    assert_eq!(addr.call(Get).await?, 2);

    addr.send(Die)?;
    sleep(Duration::from_secs(1)).await; // Wait for actor restart

    assert_eq!(addr.call(Get).await?, 0);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_broker() {
        supervise().await.expect("Supervise should work");
    }
}
