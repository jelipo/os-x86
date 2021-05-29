#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!!!");
    panic!("Panic!!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}