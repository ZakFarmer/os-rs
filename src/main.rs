#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("PANIC: {}", _info);
    loop {}
}

// Define entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("OK");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn test::Testable]) {
    use crate::qemu::{exit_qemu, QemuExitCode};

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    // Finished running tests, close QEMU
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
    assert_eq!(1, 1);
}
