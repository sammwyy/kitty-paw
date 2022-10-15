use crate::cpu::{cpu_breakpoint_handler, cpu_double_fault_handler, cpu_gdt};
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint
            .set_handler_fn(cpu_breakpoint_handler::breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(cpu_double_fault_handler::double_fault_handler)
                .set_stack_index(cpu_gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
