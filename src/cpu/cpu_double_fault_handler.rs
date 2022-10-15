use crate::{print, println, set_color, vga::vga_color::Color};
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    set_color!(Color::Red, Color::Black);
    print!("==== CPU_DOUBLE_FAULT ====");
    set_color!(Color::LightRed, Color::Black);
    println!("\n{:#?}", stack_frame);
    set_color!(Color::White, Color::Black);
    panic!("cpu_double_fault_handler");
}
