mod idt;
mod pic;

pub fn init() {
    idt::init();
    pic::init();

    x86_64::instructions::interrupts::enable();
}
