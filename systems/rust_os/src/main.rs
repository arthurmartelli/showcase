#![no_std] // don't link Rust standard library
#![no_main] // disable all Rust-level entry points

mod macros;
mod vga_buffer;

use core::panic::PanicInfo;

// function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{info}");
    loop {}
}

#[no_mangle] // ! don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello {}!", "World");

    loop {}
}
