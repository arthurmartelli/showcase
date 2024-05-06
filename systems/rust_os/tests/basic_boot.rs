// ! STD LIBRARY
#![no_std] // don't link Rust's standard library
#![no_main] // disable all Rust-level entry points
// ! TESTING
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::base::test::test_runner_handler)]
#![reexport_test_harness_main = "test_main"]

use rust_os::prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
