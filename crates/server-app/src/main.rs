// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

/// A node is a wrapper over server which is designed
/// to be launched as a system stand-alone executable
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    server_app::start().await
}
