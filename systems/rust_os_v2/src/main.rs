// ! STD LIBRARY
#![no_std] // don't link Rust's standard library
#![no_main] // disable all Rust-level entry points
// ! TESTING
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_v2::base::test::test_runner_handler)]
#![reexport_test_harness_main = "test_main"]

use rust_os_v2::prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // ! INITIALIZATION
    rust_os_v2::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

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
    rust_os_v2::base::test::test_panic_handler(info)
}
