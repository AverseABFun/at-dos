#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(at_dos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use at_dos::{serial_print, serial_println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    at_dos::test_panic_handler(info)
}


#[test_case]
fn serial_prints() {
    serial_print!("Serial print works");
}

#[test_case]
fn serial_printlns() {
    serial_println!("Serial println works");
}
