use lazy_static::lazy_static;
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

use crate::core::gdt::DOUBLE_FAULT_STACK_INDEX;

lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr({ unsafe { &STACK } });
            let stack_end = stack_start + STACK_SIZE;

            stack_end
        };

        tss
    };
}
