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
