// ! STD
#![no_std]
#![cfg_attr(test, no_main)]
// ! TESTS
#![feature(custom_test_frameworks)] // enable custom test framework
#![test_runner(crate::internals::test::test_runner_handler)] // define test runner
#![reexport_test_harness_main = "test_main"] // define test main function
// ! ABI
#![feature(abi_x86_interrupt)]

pub mod internals;
pub mod libs;
pub mod vga_buffer;

extern crate alloc;

pub fn init() {
    internals::gdt::init();
    internals::interrupts::init();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// ! TESTING
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    internals::test::test_panic_handler(info)
}

pub mod prelude {
    pub use crate::{print, println, serial_print, serial_println};
}
