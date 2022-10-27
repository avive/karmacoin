// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use std::time::Duration;

use futures::stream;
use tokio::time::sleep;
use xactor::*;

#[message(result = "i32")]
struct GetSum;

#[derive(Default)]
struct MyActor(i32);

#[async_trait::async_trait]
impl StreamHandler<i32> for MyActor {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: i32) {
        self.0 += msg;
    }

    async fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("stream started");
    }

    async fn finished(&mut self, _ctx: &mut Context<Self>) {
        println!("stream finished");
    }
}

#[async_trait::async_trait]
impl Handler<GetSum> for MyActor {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: GetSum) -> i32 {
        self.0
    }
}

#[async_trait::async_trait]
impl Actor for MyActor {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        let values = (0..100).collect::<Vec<_>>();
        ctx.add_stream(stream::iter(values));
        Ok(())
    }
}

pub async fn stream() -> Result<()> {
    let addr = MyActor::start_default().await?;
    sleep(Duration::from_secs(1)).await; // Wait for the stream to complete
    let res = addr.call(GetSum).await?;
    assert_eq!(res, (0..100).sum::<i32>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_steam() {
        stream().await.expect("Stream should work");
    }
}
