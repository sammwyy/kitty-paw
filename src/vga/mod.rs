pub mod vga_buffer;
pub mod vga_color;
pub mod vga_color_mode;
pub mod vga_screen_char;
pub mod vga_writer;

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

use self::vga_color::Color;

lazy_static! {
    pub static ref WRITER: Mutex<vga_writer::Writer> = Mutex::new(vga_writer::Writer {
        column_position: 0,
        color_code: vga_color_mode::ColorMode::new(
            vga_color::Color::White,
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

#[macro_export]
macro_rules! set_color {
    ($fg:expr, $bg:expr) => {
        $crate::vga::_set_color($fg, $bg)
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _set_color(fg: Color, bg: Color) {
    WRITER.lock().set_color(fg, bg);
}
