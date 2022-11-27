use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

use super::{
    colour::{Colour, ColourCode},
    text::{CharBuffer, RenderedChar},
    BUFFER_HEIGHT, BUFFER_WIDTH,
};

lazy_static! {
    pub static ref PRINTER: Mutex<Printer> = Mutex::new(Printer {
        cursor_x: 0,
        cursor_y: 0,
        colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
        char_buffer: unsafe { &mut *(0xb8000 as *mut CharBuffer) },
    });
}

pub struct Printer {
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub colour_code: ColourCode,
    pub char_buffer: &'static mut CharBuffer,
}

impl Printer {
    pub fn print_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.cursor_x >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.cursor_x;

                let colour_code = ColourCode::new(Colour::White, Colour::Black);
                self.char_buffer.chars[row][col].write(RenderedChar {
                    ascii_char: byte,
                    colour_code,
                });
                self.cursor_x += 1;
            }
        }
    }

    pub fn print_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                // Printable ASCII
                0x20..=0x7e | b'\n' => self.print_byte(byte),
                _ => self.print_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.char_buffer.chars[row][col].read();
                self.char_buffer.chars[row - 1][col].write(char);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.cursor_x = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = RenderedChar {
            ascii_char: b' ',
            colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
        };

        for col in 0..BUFFER_WIDTH {
            self.char_buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Printer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.print_string(string);
        Ok(())
    }
}

// Define custom macros for printing to the screen
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::printer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    PRINTER.lock().write_fmt(args).unwrap();
}

#[test_case]
fn test_println_single() {
    println!("Testing single println");
}

#[test_case]
fn test_println_many() {
    for _ in 0..100 {
        println!("Testing many printlns");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Testing single println";
    println!("{}", s);

    for (i, c) in s.chars().enumerate() {
        let rendered_char = PRINTER.lock().char_buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(rendered_char.ascii_char), c);
    }
}
