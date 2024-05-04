// ! STD LIBRARY
#![no_std] // don't link Rust's standard library
#![no_main] // disable all Rust-level entry points

// ! TEST FRAMEWORK
#![feature(custom_test_frameworks)] // enable custom test framework
#![test_runner(crate::test_runner)] // define test runner
#![reexport_test_harness_main = "test_main"] // define test main function

mod qemu;
mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{info}");

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker
    // looks for a function  named `_start` by default
    println!("Hello {}!", "world");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::qemu::{exit_qemu, QemuExitCode};

    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success)
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::qemu::{exit_qemu, QemuExitCode};

    serial_println!("[failed]");
    serial_println!("Error: {info}");
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

#[test_case]
fn trivial() {
    serial_print!("trivial test... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}
