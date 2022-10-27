// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use std::time::Duration;
use tokio::time::sleep;
use xactor::*;

#[message]
#[derive(Clone)]
struct MyMsg(&'static str);

#[message(result = "String")]
struct GetValue;

#[derive(Default)]
struct MyActor(String);

#[async_trait::async_trait]
impl Actor for MyActor {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        ctx.subscribe::<MyMsg>().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Handler<MyMsg> for MyActor {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: MyMsg) {
        self.0 += msg.0;
    }
}

#[async_trait::async_trait]
impl Handler<GetValue> for MyActor {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: GetValue) -> String {
        self.0.clone()
    }
}

pub async fn broker() -> Result<()> {
    let addr1 = MyActor::start_default().await?;
    let addr2 = MyActor::start_default().await?;

    Broker::from_registry().await?.publish(MyMsg("a"))?;
    Broker::from_registry().await?.publish(MyMsg("b"))?;

    sleep(Duration::from_secs(1)).await; // Wait for the messages

    assert_eq!(addr1.call(GetValue).await?, "ab");
    assert_eq!(addr2.call(GetValue).await?, "ab");
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_broker() {
        broker().await.expect("broker should work");
    }
}
