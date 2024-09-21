#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)] // Test frameworks not linked to std
#![test_runner(crate::test_runner)] // Defining test runner
#![reexport_test_harness_main = "test_main"] // Tests will create a test_main function now

// Only run when 'cargo test' called
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("\nRunning {} tests", tests.len());
    for test in tests {
        test();
    }
    println!();
}

// Test Cases
#[test_case]
fn trivial_assertion() {
    print!("Trivial Assertion: ");
    assert_eq!(1, 1);
    println!(" ..[OK]");
}

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
    // Call all the tests if 'cargo test' called
    #[cfg(test)]
    test_main();

    println!("Hello, World! {}", "(Once)");
    println!("{}Hello, World!", "[OK]");
    println!("{}Hello, World!", "[ERR]");
    println!("Hello, World! {}", "To this whole world!!!");

    panic!("This is some panic message");
    loop {}
}
