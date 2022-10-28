// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

/// A node is a wrapper over server which is designed
/// to be launched as a system stand-alone executable
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    server_app::start().await
}
