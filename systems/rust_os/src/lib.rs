#![no_std]
#![cfg_attr(test, no_main)]
// ! TESTS
#![feature(custom_test_frameworks)] // enable custom test framework
#![test_runner(crate::test::test_runner_handler)] // define test runner
#![reexport_test_harness_main = "test_main"] // define test main function

pub mod qemu;
pub mod serial;
pub mod test;
pub mod vga_buffer;

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test::test_panic_handler(info)
}
