#![no_std] // Don't link rust std library.
#![no_main] // Disable rust-level entry points.

mod vga;

use core::panic::PanicInfo;

// Entrypoint since the linker looks for a function
// named '_start' by default.

#[no_mangle] // Don't mangle the name of this fn.
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Intentional crash");

    // Block with a infinite loop.
    // loop {}
}

// Function called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernal Panic: {}", info);
    loop {}
}
