#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(raw_ref_op)]
#![test_runner(rs_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rs_os::{idt, println, print};
use x86_64::instructions::hlt;

// Define entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("[Kernel] Initialising...");

    rs_os::init();

    println!("[Kernel] Initialised.");

    #[cfg(test)]
    test_main();

    rs_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    
    rs_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rs_os::test_panic_handler(info)
}
