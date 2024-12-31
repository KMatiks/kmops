#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(kmops::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kmops::{println, memory::active_level_4_table};
use x86_64::VirtAddr;
use bootloader::{BootInfo, entry_point};


entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    kmops::init();

    println!("It didn't crash!!!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();

    kmops::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kmops::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kmops::test_panic_handler(info);
}
