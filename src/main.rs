#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// this function is the entry point, since the linker looks for a function
// named `_start` by default
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // VGA Buffer located at 0xb8000 (convert to raw pointer)

    for (i, &byte) in HELLO.iter().enumerate() {
        // Absolutely sure that this works (not the way to do it in rust)
        // If doing this, its not different than doing this in c
        unsafe {
            // Offset method to write to
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Color (Light Cyan)
        }
    }
    loop {}
}
