use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const PAGE_SIZE: usize = 4096;
pub const STACK_SIZE: usize = PAGE_SIZE * 5;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

/*
addr
----
High   +----------------------------+
       |                            |
       |   Unused Stack Space       |
       |                            |
       +----------------------------+ <-- stack_end (Highest Address)
       |                            |
       |   Allocated Stack Memory   |
       |     Growing Downwards      |
       |            |               |
       |            |               |
       |            v               |
       +----------------------------+ <-- stack_start (Lowest Address)
Low    |                            |

Memory Layout:
[High Address] stack_end   ←── Top of Stack
                ↓
                Stack Data
                ↓
[Low Address]  stack_start ←── Bottom of Stack
*/

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    GDT.0.load();

    unsafe {
        // Reload the code segment register to use new GDT
        CS::set_reg(GDT.1.code_selector);
        // Load the TSS
        load_tss(GDT.1.tss_selector);
    }
}