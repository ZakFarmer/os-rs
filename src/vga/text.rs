use volatile::Volatile;

use super::{colour::ColourCode, BUFFER_HEIGHT, BUFFER_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RenderedChar {
    pub ascii_char: u8,
    pub colour_code: ColourCode,
}

#[repr(transparent)]
pub struct CharBuffer {
    pub chars: [[Volatile<RenderedChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
