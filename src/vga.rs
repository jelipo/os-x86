#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrontColor {
    Black = 0b_0000_0000,
    Blue = 0b_0001_0000,
    Green = 0b_0010_0000,
    Cyan = 0b_0011_0000,
    Red = 0b_0100_0000,
    Magenta = 0b_0101_0000,
    Brown = 0b_0110_0000,
    LightGray = 0b_0111_0000,
    DarkGray = 0b_1000_0000,
    LightBlue = 0b_1001_0000,
    LightGreen = 0b_1010_0000,
    LightCyan = 0b_1011_0000,
    LightRed = 0b_1100_0000,
    Pink = 0b_1101_0000,
    Yellow = 0b_1110_0000,
    White = 0b_1111_0000,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BackColor {
    Black = 0b_0000_0000,
    Blue = 0b_0000_0010,
    Green = 0b_0000_0100,
    Cyan = 0b_0000_0110,
    Red = 0b_0000_1000,
    Magenta = 0b_0000_1010,
    Brown = 0b_0000_1100,
    LightGray = 0b_0000_1110,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(front_color: FrontColor, back_color: BackColor) -> ColorCode {
        ColorCode((front_color as u8) << 4 | (back_color as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}