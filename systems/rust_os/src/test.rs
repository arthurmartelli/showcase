use crate::serial_print;
use crate::serial_println;

#[cfg(test)]
pub fn runner(tests: &[&dyn Testable]) {
    use crate::qemu::{exit_qemu, QemuExitCode};

    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success)
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use crate::qemu::{exit_qemu, QemuExitCode};

    serial_println!("[failed]");
    serial_println!("Error: {info}");
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

pub trait Testable {
    #[allow(dead_code)]
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
