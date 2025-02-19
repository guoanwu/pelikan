// Copyright 2021 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

#[macro_use]
extern crate logger;

use backtrace::Backtrace;
use config::SegcacheConfig;
use pelikan_segcache_rs::Segcache;

fn main() {
    // custom panic hook to terminate whole process after unwinding
    std::panic::set_hook(Box::new(|s| {
        error!("{}", s);
        println!("{:?}", Backtrace::new());
        std::process::exit(101);
    }));

    // load config from file
    let config = if let Some(file) = std::env::args().nth(1) {
        debug!("loading config: {}", file);
        match SegcacheConfig::load(&file) {
            Ok(c) => c,
            Err(e) => {
                error!("{}", e);
                std::process::exit(1);
            }
        }
    } else {
        Default::default()
    };

    // launch segcache
    Segcache::new(config).wait()
}
