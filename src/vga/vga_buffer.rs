use volatile::Volatile;

use super::vga_screen_char::ScreenChar;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
