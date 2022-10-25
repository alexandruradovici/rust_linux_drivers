// SPDX-License-Identifier: GPL-2.0
//! Rust Hello World

// The rust driver does not link to the standard rust
// library, which means that need to manually import
// data tyes like `Result`.
// This also imports the data type `ThisModule` and
// the `module!` macro.
use kernel::prelude::*;

// Define the kernel driver information
module! {
    type: Driver,
    name: b"hello_world",
    author: b"Alexandru Radovici",
    description: b"Rust driver that prints hello world",
    license: b"GPL",
}

/// The structure that represents our driver.
///
/// As this is a very simple driver,
/// this is an empty structure.
struct Driver {}

/// op be able to register the `Driver` structure as
/// a linux driver, it needs the implement the `Module`
/// trait.
impl kernel::Module for Driver {
    /// This function is called when the driver is initialized
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World loaded\n");
        pr_info!("Hello World\n");
        Ok(Driver {})
    }
}
impl Drop for Driver {
    /// This function is called when the driver unloaded
    fn drop(&mut self) {
        pr_info!("Hello World unloaded\n");
    }
}
