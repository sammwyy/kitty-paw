pub mod vga_buffer;
pub mod vga_color;
pub mod vga_color_mode;
pub mod vga_screen_char;
pub mod vga_writer;

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<vga_writer::Writer> = Mutex::new(vga_writer::Writer {
        column_position: 0,
        color_code: vga_color_mode::ColorMode::new(
            vga_color::Color::Yellow,
            vga_color::Color::Black
        ),
        buffer: unsafe { &mut *(0xb8000 as *mut vga_buffer::Buffer) },
    });
}

// Macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
