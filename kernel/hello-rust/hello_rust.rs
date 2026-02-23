// SPDX-License-Identifier: GPL-2.0

#![allow(missing_docs)]

use kernel::prelude::*;

module! {
    type: HelloRust,
    name: "hello_rust",
    authors: ["Slowboot"],
    description: "hello rust example",
    license: "GPL",
}

struct HelloRust;

impl kernel::Module for HelloRust {
	fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello rust world!\n");

        Ok(HelloRust)
    }
}

impl Drop for HelloRust {
    fn drop(&mut self) {
        pr_info!("Have a nice day!\n");
    }
}
