use crate::{print, println, set_color, vga::vga_color::Color};
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    set_color!(Color::Red, Color::Black);
    print!("===== CPU_BREAKPOINT =====");
    set_color!(Color::LightRed, Color::Black);
    println!("\n{:#?}", stack_frame);
    set_color!(Color::White, Color::Black);
}
