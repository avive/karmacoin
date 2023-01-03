// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

// test helper functions

//use env_logger::Target;

use log::LevelFilter;

pub fn enable_logger() {
    let _ = env_logger::builder()
        .is_test(false)
        //.target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .try_init();
}
