#![no_std]
#![no_main]


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // This function is called on panic
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This function is the entry point
    loop {}
}
