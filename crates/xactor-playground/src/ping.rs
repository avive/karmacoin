// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use xactor::*;

/// Define `Ping` message
#[message(result = "usize")]
struct Ping(usize);

/// Actor
struct PingActor {
    count: usize,
}

/// Declare actor and its context
impl Actor for PingActor {}

/// Handler for `Ping` message
#[async_trait::async_trait]
impl Handler<Ping> for PingActor {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Ping) -> usize {
        self.count += msg.0;
        self.count
    }
}

pub async fn ping() -> Result<()> {
    // start new actor
    let addr = PingActor { count: 10 }.start().await?;

    // send message and get future for result
    let res = addr.call(Ping(10)).await?;
    println!("RESULT: {}", res == 20);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ping() {
        ping().await.expect("ping should work");
    }
}
