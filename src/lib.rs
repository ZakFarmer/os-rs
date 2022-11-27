#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod qemu;
mod serial;
mod test;
pub mod vga;

use core::panic::PanicInfo;

use crate::qemu::exit_qemu;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

pub fn test_runner(tests: &[&dyn test::Testable]) {
    use crate::qemu::{exit_qemu, QemuExitCode};

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    // Finished running tests, close QEMU
    qemu::exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);

    qemu::exit_qemu(qemu::QemuExitCode::Fail);
    loop {}
}


#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}