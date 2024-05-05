// ! STD LIBRARY
#![no_std] // don't link Rust's standard library
#![no_main] // disable all Rust-level entry points
// ! TESTING
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::test_runner_handler)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test::test_panic_handler(info)
}
