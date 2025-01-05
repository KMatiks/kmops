use bootloader::BootInfo;
use x86_64::{
    structures::paging::{PageTable, OffsetPageTable},
    VirtAddr,
    PhysAddr
};
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PhysFrame, Size4KiB};
use bootloader::bootinfo::{MemoryMap};
use bootloader::bootinfo::MemoryRegionType;

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // Obtain an iterator to the memory regions of the memory map
        let regions = self.memory_map.iter();
        // Determine which of those regions are usable by the kernel
        let usable_regions = regions.
            filter(|r| r.region_type == MemoryRegionType::Usable);
        // Obtain a map of the address range for each usable region
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        // Flatten all address ranges into a continuous mapping of 4 KiB chunks
        let frame_address = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_address.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub unsafe fn init(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    let pml4_table = active_level_4_table(phys_mem_offset);
    OffsetPageTable::new(pml4_table, phys_mem_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}
