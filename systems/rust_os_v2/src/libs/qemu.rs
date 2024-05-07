#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    // 0xf4 is the port number of the QEMU exit code register
    let mut port = Port::new(0xf4);

    unsafe {
        port.write(exit_code as u32);
    }
}
