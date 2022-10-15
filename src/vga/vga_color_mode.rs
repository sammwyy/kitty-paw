use super::vga_color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorMode(u8);

impl ColorMode {
    pub fn new(foreground: Color, background: Color) -> ColorMode {
        ColorMode((background as u8) << 4 | (foreground as u8))
    }
}
