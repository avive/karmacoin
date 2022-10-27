// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use xactor::*;

#[message(result = "i32")]
struct AddMsg(i32);

#[derive(Default)]
struct MyService(i32);

impl Actor for MyService {}

impl Service for MyService {}

#[async_trait::async_trait]
impl Handler<AddMsg> for MyService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: AddMsg) -> i32 {
        self.0 += msg.0;
        self.0
    }
}

pub async fn service() -> Result<()> {
    let addr = MyService::from_registry().await?;
    assert_eq!(addr.call(AddMsg(1)).await?, 1);
    assert_eq!(addr.call(AddMsg(5)).await?, 6);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_service() {
        service().await.expect("service should work");
    }
}
