#![no_std] // Don't link rust std library.
#![no_main] // Disable rust-level entry points.
#![feature(abi_x86_interrupt)] // Use x86-interrupt calling convention.

mod cpu;
mod vga;

use cpu::cpu_interrupt_index::PICS;

use crate::vga::vga_color::Color;
use core::panic::PanicInfo;

// CPU Halting
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// Initialize kernel
pub fn init() {
    cpu::cpu_gdt::init();
    cpu::cpu_interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

// Entrypoint since the linker looks for a function
// named '_start' by default.

#[no_mangle] // Don't mangle the name of this fn.
pub extern "C" fn _start() -> ! {
    init();

    // Print hello world in screen.
    println!("Hello World{}", "!");

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    // Access memory out of kernel.
    /*
    let ptr = 0xdeadbeaf as *mut u32;
    unsafe {
        *ptr = 42;
    }
    */

    // Trigger exception.
    /*
       unsafe {
           *(0xdeadbeef as *mut u64) = 42;
       };
    */

    // Make it crash.
    /*
        x86_64::instructions::interrupts::int3();
        panic!("Intentional crash");
    */

    // Block with a infinite loop.
    hlt_loop();
}

// Function called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_color!(Color::Red, Color::Black);
    print!("Kernal Panic: ");
    set_color!(Color::LightRed, Color::Black);
    print!("{}", info);

    // Block with a infinite loop.
    hlt_loop();
}
