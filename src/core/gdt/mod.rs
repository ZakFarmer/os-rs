use lazy_static::lazy_static;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

mod tss;

pub const DOUBLE_FAULT_STACK_INDEX: u16 = 0;

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&tss::TSS));

        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    GDT.0.load();

    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
