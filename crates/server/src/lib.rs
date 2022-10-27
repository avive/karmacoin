// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
extern crate base;
extern crate db;

// used by server-app to start the server
pub use services::server_service;

mod services;
