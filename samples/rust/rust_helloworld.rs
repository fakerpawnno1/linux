
// SPDX-License-Identifier: GPL-2.0

//! hello world module in rust

use kernel::prelude::*;
use kernel::bindings::ww_kernel_test;
module! {
    type: RustHelloWorld,
    name: "rust_helloworld",
    author: "Rust for Linux Contributors",
    description: "hello world module in rust",
    license: "GPL",
}

struct RustHelloWorld;

impl kernel::Module for RustHelloWorld {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World from Rust module (init)\n");
      unsafe{  ww_kernel_test();}
        Ok(RustHelloWorld)
    }
}

impl Drop for RustHelloWorld {
    fn drop(&mut self) {
        pr_info!("Hello World from Rust module (exit)\n");
    }
}
