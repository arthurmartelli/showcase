#![no_std] // don't link Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use crate::vga_buffer::WRITER;
use core::panic::PanicInfo;

// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // ! don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    use core::fmt::Write;

    WRITER.lock().write_str("Hello World!").unwrap();
    write!(WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}
