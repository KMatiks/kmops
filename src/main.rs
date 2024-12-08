#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(kmops::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kmops::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("1");
    println!("2");
    println!("3");

    kmops::init();

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }

    x86_64::instructions::interrupts::int3();

    println!("It didn't crash!!!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
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
