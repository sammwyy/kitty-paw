#![no_std] // Don't link rust std library.
#![no_main] // Disable rust-level entry points.
#![feature(abi_x86_interrupt)] // Use x86-interrupt calling convention.

mod cpu;
mod memory;
mod vga;

use bootloader::{entry_point, BootInfo};
use cpu::cpu_interrupt_index::PICS;

use core::panic::PanicInfo;
use vga::vga_color::Color;

use crate::memory::{mem_boot_info_frame_allocator::BootInfoFrameAllocator, mem_example_mapping};

// CPU Halting
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// Logging
pub fn print_ok_proc(msg: &str) {
    set_color!(Color::Green, Color::Black);
    print!("  [OK] ");
    set_color!(Color::LightGreen, Color::Black);
    println!("{}", msg);
    set_color!(Color::White, Color::Black);
}

pub fn print_info_sub(msg: &str) {
    set_color!(Color::DarkGray, Color::Black);
    print!("    > ");
    set_color!(Color::LightGray, Color::Black);
    println!("{}", msg);
    set_color!(Color::White, Color::Black);
}

pub fn print_info(msg: &str) {
    set_color!(Color::Yellow, Color::Black);
    println!("{}", msg);
    set_color!(Color::White, Color::Black);
}

// Initialize kernel
pub fn init() {
    print_info("Initializing kernel...");
    cpu::cpu_gdt::init();
    print_ok_proc("init          cpu::cpu_gdt");
    cpu::cpu_interrupts::init_idt();
    print_ok_proc("init_idt      cpu::cpu_interrupts");
    unsafe { PICS.lock().initialize() };
    print_ok_proc("initialize    PICS.lock()");
    x86_64::instructions::interrupts::enable();
    print_ok_proc("enable        x86_64::instructions::interrupts");
}

// Test memory mapping
pub fn test_memory_map(boot_info: &'static BootInfo) {
    print_info("Testing memory mapping");

    use x86_64::{structures::paging::Page, VirtAddr};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    print_ok_proc("new(addr)     x86_64::VirtAddr::new");

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    print_ok_proc("init(pmo)     memory");

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    print_ok_proc("init(mm)      mem_boot_info_frame_allocator::BootInfoFrameAllocator");

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    mem_example_mapping::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(220).write_volatile(0x_f074_f06d_f065_f04d) };
    unsafe { page_ptr.offset(221).write_volatile(0x_f021_f074_f073_f065) };

    print_ok_proc("write_volatile page_ptr.offset");
    print_info_sub("count   100");
    print_info_sub("val     0x f074 f06d f065 f04d");

    print_ok_proc("write_volatile page_ptr.offset");
    print_info_sub("count   101");
    print_info_sub("val     0x f021 f074 f073 f065");
}

// Entry point
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init();
    test_memory_map(boot_info);

    print_info("\nAll tests passed, kitty_paw is loaded.\n");
    print!("Hello World > ");

    hlt_loop();
}

entry_point!(kernel_main);

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
