#![no_std] // Don't link rust std library.
#![no_main] // Disable rust-level entry points.

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Kitty Paw";

#[no_mangle] // Don't mangle the name of this fn.
pub extern "C" fn _start() -> ! {
    // Entrypoint since the linker looks for a function
    // named '_start' by default.

    // VGA rendering.
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xd;
        }
    }

    // Block with a infinite loop.
    loop {}
}

// Function called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
