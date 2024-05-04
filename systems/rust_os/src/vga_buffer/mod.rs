mod color;
mod writer;

use color::{Color, ColorCode};
use lazy_static::lazy_static;
// spin-lock mutex to add interior mutability
use spin::mutex::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<writer::Writer> = Mutex::new(writer::Writer::new(
        0,
        ColorCode::new(Color::Yellow, Color::Black),
        unsafe { &mut *(0xb8000 as *mut writer::Buffer) }
    ));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/// Prints to the host through the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)))
}

/// Prints to the host through the VGA text buffer, appending a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}
