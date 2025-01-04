use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};
use x86_64::structures::paging::{Page, PageTableFlags};
use crate::{println};

pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

pub unsafe fn calculate_memory_usage(physical_memory_offset: VirtAddr) -> () {
    let pml4_table = active_level_4_table(physical_memory_offset);

    const MAX_ENTRIES: u16 = 1 << 9;
    const MAX_L3_ENTRIES: u64 = 1 << 18;
    const MAX_L2_ENTRIES: u64 = 1 << 27;
    const MAX_PAGE_COUNT: u64 = 1 << 36;

    let mut num_l4_unused: u16 = 0;
    let mut num_l3_unused: u64 = 0;
    let mut num_l2_unused: u64 = 0;
    let mut num_l1_unused: u64 = 0;

    for (i, entry) in pml4_table.iter().enumerate() {
        if entry.is_unused() {
            num_l4_unused += 1;
            continue;
        }

        let phys = entry.frame().unwrap().start_address();
        let va = physical_memory_offset + phys.as_u64();
        let ptr = va.as_mut_ptr();
        let l3_table: &PageTable = unsafe { &*ptr };

        for (i, entry) in l3_table.iter().enumerate() {
            if entry.is_unused() {
                num_l3_unused += 1;
                continue;
            }

            if !entry.is_unused() {
                let pa = entry.frame().unwrap().start_address();
                let va = physical_memory_offset + pa.as_u64();
                let ptr = va.as_mut_ptr();
                let l2_table: &PageTable = unsafe { &*ptr };

                for (i, entry) in l2_table.iter().enumerate() {
                    if entry.is_unused() {
                        num_l2_unused += 1;
                        continue;
                    }

                    if entry.flags().contains(PageTableFlags::HUGE_PAGE) {
                        continue;
                    }

                    let pa = entry.frame().unwrap().start_address();
                    let va = physical_memory_offset + pa.as_u64();
                    let ptr = va.as_mut_ptr();
                    let l1_table: &PageTable = unsafe { &*ptr };

                    for (i, entry) in l1_table.iter().enumerate() {
                        if entry.is_unused() {
                            num_l1_unused += 1;
                            continue;
                        }
                    }
                }
            }
        }
    }

    println!("{}/512 L4 entries used", MAX_ENTRIES - num_l4_unused);
    println!("{}/{} L3 entries used", MAX_L3_ENTRIES - num_l3_unused, MAX_L3_ENTRIES);
    println!("{}/{} L2 entries used", MAX_L2_ENTRIES - num_l2_unused, MAX_L2_ENTRIES);
    println!("{}/{} L1 entries used", MAX_PAGE_COUNT - num_l1_unused, MAX_PAGE_COUNT);

}