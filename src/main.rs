#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(kmops::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use kmops::{println};
use bootloader::{BootInfo, entry_point};
use alloc::boxed::Box;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    kmops::init();

    let x = Box::new(41);

    for entry in boot_info.memory_map.iter() {
        println!("Memory region: {:?}", entry);
    }

    #[cfg(test)]
    test_main();

    println!("It didn't crash!!!");

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
