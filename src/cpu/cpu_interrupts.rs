use crate::{cpu::cpu_gdt, print, println, set_color, vga::vga_color::Color};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(cpu_gdt::DOUBLE_FAULT_IST_INDEX);
        }
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

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("CPU_INTERRUPT_EXCEPTION: {:#?}", stack_frame);
}
