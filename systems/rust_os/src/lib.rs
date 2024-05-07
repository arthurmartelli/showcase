#![no_std]
#![cfg_attr(test, no_main)]
// ! TESTS
#![feature(custom_test_frameworks)] // enable custom test framework
#![test_runner(crate::base::test::test_runner_handler)] // define test runner
#![reexport_test_harness_main = "test_main"] // define test main function

pub mod base;
pub mod libs;
pub mod vga_buffer;

// ! TESTING
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    base::test::test_panic_handler(info)
}

pub mod prelude {
    pub use crate::{print, println, serial_print, serial_println};
}
