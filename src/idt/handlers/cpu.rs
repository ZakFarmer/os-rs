use x86_64::structures::idt::InterruptStackFrame;

use crate::{println, print, idt::{PICS, InterruptIndex}};

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) -> () {
    println!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT\n{:#?}\nERROR CODE: {:?}",
        stack_frame, error_code
    );
}

pub extern "x86-interrupt" fn timer_interrupt_handler(
    stack_frame: InterruptStackFrame,
) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}