#![no_std]
#![no_main]

mod vga_buffer; // Create a Rust module to handle printing

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// This function is the entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop{}
}
