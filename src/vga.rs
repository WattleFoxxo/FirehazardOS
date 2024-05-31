use core::fmt;
use lazy_static::lazy_static; /// broooooo i swear thats meeee -charlie
use spin::Mutex;
use volatile::Volatile; // POV: my brain when i do OS dev -charlie

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        colour_code: ColourCode::new(Colour::White, Colour::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);

impl ColourCode {
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,// bruh  -charlie 
    colour_code: ColourCode, // mmmmmmmm, like i HAVENT WORKED WITH THESE OVER AT THE C# REPO BEFORE -charlie
}

const BUFFER_HEIGHT: usize = 25; // FUCK -charlie
const BUFFER_WIDTH: usize = 80; // actualy, wait hold on? -charlie
// hello World
#[repr(transparent)]// screen buffer for handeling the pixels that we draw to THE FUCKING VGA DRIVER -charlie
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT], // volatile? -charlie
}

pub struct Writer { // can i fuck the vga driver? -charlie
    column_position: usize,
    row_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl Writer { // well this of course writes something.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position; // BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let colour_code = self.colour_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    colour_code,
                });

                self.column_position += 1;
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) { // i think makes a new line?
        self.row_position += 1;
        self.column_position = 0;

        if self.row_position >= BUFFER_HEIGHT {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }

            self.clear_row(BUFFER_HEIGHT - 1);
            self.row_position -= 1;
        }
    }

    fn clear_row(&mut self, row: usize) { // clear ... something?
        let blank = ScreenChar {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank); // i'm gonna fucking kill you @wattlefoxxo
        }
    }
}

impl fmt::Write for Writer { // ooh a implementation for something? - charlie
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
