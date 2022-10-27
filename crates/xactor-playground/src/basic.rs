// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use xactor::*;

#[message(result = "String")]
struct ToUppercase(String);

struct BasicActor;

impl Actor for BasicActor {}

#[async_trait::async_trait]
impl Handler<ToUppercase> for BasicActor {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: ToUppercase) -> String {
        msg.0.to_uppercase()
    }
}

pub async fn basic() -> Result<()> {
    // Start actor and get its address
    let addr = BasicActor.start().await?;

    // Send message `ToUppercase` to actor via addr
    let res = addr.call(ToUppercase("lowercase".to_string())).await?;
    println!("{}", res);
    assert_eq!(res, "LOWERCASE");
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_basic() {
        basic().await.expect("basic should work");
    }
}
