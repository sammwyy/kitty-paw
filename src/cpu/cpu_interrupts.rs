use crate::cpu::{
    cpu_breakpoint_handler, cpu_double_fault_handler::double_fault_handler,
    cpu_gdt::DOUBLE_FAULT_IST_INDEX, cpu_interrupt_index::InterruptIndex,
    cpu_keyboard_interrupt_handler::keyboard_interrupt_handler,
    cpu_timer_interrupt_handler::timer_interrupt_handler,
};
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint
            .set_handler_fn(cpu_breakpoint_handler::breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
