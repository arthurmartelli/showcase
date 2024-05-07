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

    #[allow(clippy::empty_loop)]
    loop {}
}

// ! PANIC HANDLER
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rust_os::base::test::test_panic_handler(info)
}
