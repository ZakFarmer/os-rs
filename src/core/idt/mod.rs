use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::core::gdt::DOUBLE_FAULT_STACK_INDEX;

mod handlers;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Attach breakpoint interrupt handler
        idt.breakpoint.set_handler_fn(handlers::cpu::breakpoint_handler);

        // Attach divide by zero interrupt handler
        idt.divide_error
            .set_handler_fn(handlers::cpu::divide_by_zero_handler);

        // Attach double fault interrupt handler
        unsafe {
            idt.double_fault
                .set_handler_fn(handlers::cpu::double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_STACK_INDEX);
        }

        // Attach PS/2 keyboard interrupt handler
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(handlers::keyboard::keyboard_interrupt_handler);

        // Attach timer interrupt handler
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(handlers::cpu::timer_interrupt_handler);

        idt
    };
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init() -> () {
    IDT.load();
}
