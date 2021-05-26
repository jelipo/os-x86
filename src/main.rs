#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

use crate::vga::Writer;

mod vga;

static HELLO: &[u8] = b"Hello World dsadasdas dasda !\n";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new();
    for i in 0..1024 {
        writeln!(writer, "Hello World!{}", i);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}