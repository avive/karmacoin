// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
extern crate base;
extern crate core;
extern crate db;

// used by server-app to start the server
pub use services::blockchain::tokenomics::Tokenomics;
pub use services::server_service;
mod services;
