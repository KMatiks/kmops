#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("1");
    println!("2");
    println!("3");
    loop {}
}
