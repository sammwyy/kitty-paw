use crate::{print, println, set_color, vga::vga_color::Color};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    set_color!(Color::Red, Color::Black);
    print!("===== CPU_INTERRUPT_EXCEPTION =====");
    set_color!(Color::LightRed, Color::Black);
    println!("\n{:#?}", stack_frame);
    set_color!(Color::White, Color::Black);
}
