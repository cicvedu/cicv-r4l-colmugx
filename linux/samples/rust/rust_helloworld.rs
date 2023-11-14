// SPDX-License-Identifier: GPL-2.0
//! Rust minimal sample.

use kernel::prelude::*;

module! {
    type: RustHelloWorld,
    name: "rust_helloworld",
    author: "your_dad",
    description: "hello world descriptions",
    license: "GPL",
}

struct RustHelloWorld;

impl kernel::Module for RustHelloWorld {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World from Rust module");
        Ok(RustHelloWorld)
    }
}
