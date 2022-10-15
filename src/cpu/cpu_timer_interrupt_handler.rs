use x86_64::structures::idt::InterruptStackFrame;

use crate::cpu::cpu_interrupt_index::{InterruptIndex, PICS};

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
