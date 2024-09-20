#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// Mods
mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// this function is the entry point, since the linker looks for a function
// named `_start` by default
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello, World! {}", "(Once)");
    println!("{}Hello, World!", "[OK]");
    println!("{}Hello, World!", "[ERR]");
    println!("Hello, World! {}", "To this whole world!!!");
    panic!("This is some panic message");

    loop {}
}
