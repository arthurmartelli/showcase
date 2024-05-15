// ! STD LIBRARY
#![no_std] // don't link Rust's standard library
#![no_main] // disable all Rust-level entry points
// ! TESTING
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_v2::internals::test::test_runner_handler)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use rust_os_v2::prelude::*;

entry_point!(main);

#[no_mangle]
fn main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rust_os_v2::internals::test::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
