// ! STD LIBRARY
#![no_std] // don't link Rust's standard library
#![no_main] // disable all Rust-level entry points

// ! TEST FRAMEWORK
#![feature(custom_test_frameworks)] // enable custom test framework
#![test_runner(crate::test::runner)] // define test runner
#![reexport_test_harness_main = "test_main"] // define test main function

mod qemu;
mod serial;
mod test;
mod vga_buffer;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("{info}");

    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker
    // looks for a function  named `_start` by default
    println!("Hello {}!", "world");

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[test_case]
fn trivial() {
    assert_eq!(1, 1);
}
