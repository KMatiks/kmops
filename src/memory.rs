use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};

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
    
    const MAX_ENTRIES: u16 = 512;
    const PAGES_PER_L4_ENTRY: u32 = 1<<27;
    const TOTAL_PAGES: u64 = 1<<36;

    let mut num_l4_unused = 0;
    let mut num_l3_unused = 0;
    let mut num_l2_unused = 0;

    for (i, entry) in pml4_table.iter().enumerate() {
        if entry.is_unused() {
            num_l4_unused += 1;
        }

        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            let phys = entry.frame().unwrap().start_address();
            let ptr = physical_memory_offset + phys.as_u64();
            let l3_table: &PageTable = unsafe { &*ptr };

            for (i, entry) in l3_table.iter().enumerate() {
                println!("L3 Entry {}: {:?}", i, entry);

                if !entry.is_unused() {
                    let pa = entry.frame().unwrap().start_address();
                    let va = physical_memory_offset + pa.as_u64();
                    let ptr = VirtAddr::new(va).as_mut_ptr();
                    let l4_table: &PageTable = unsafe { &*ptr };

                    for (i, entry) in l4_table.iter().enumerate() {
                        println!("L4 Entry {}: {:?}", i, entry);
                    }
                }
            }
        }


    }

    println!("{}/512 L4 entries unused", num_l4_unused);
    println!("{}/512 L4 entries used", MAX_ENTRIES - num_l4_unused);
}