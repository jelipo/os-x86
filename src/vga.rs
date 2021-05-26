use core::fmt;
use core::fmt::Write;

pub static WRITER: Writer = Writer {
    column_position: 0,
    row_position: 0,
    color_code: ColorCode::new(VgaColor::White, VgaColor::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
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
struct ColorCode(u8);

impl ColorCode {
    fn new(front_color: VgaColor, back_color: VgaColor) -> ColorCode {
        ColorCode((back_color as u8) << 4 | (front_color as u8))
    }
}

/// transparent 让数据在内存中按照预期，不能切换顺序
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
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(VgaColor::White, VgaColor::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.next_line(),
            _ => {
                if self.column_position == BUFFER_WIDTH {
                    self.next_line();
                }
                let row = self.row_position;
                let col = self.column_position;
                self.buffer.chars[row][col] = ScreenChar { ascii_character: byte, color_code: self.color_code };
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn next_line(&mut self) {
        if self.row_position == (BUFFER_HEIGHT - 1) {
            self.up_line();
        } else {
            self.row_position += 1;
        }
        self.column_position = 0;
    }

    fn up_line(&mut self) {
        for row in 0..(BUFFER_HEIGHT - 1) {
            for i in 0..BUFFER_WIDTH {
                self.buffer.chars[row][i] = self.buffer.chars[row + 1][i];
            }
        }
        self.clean_line(BUFFER_HEIGHT - 1);
    }

    fn clean_line(&mut self, row: usize) {
        for i in 0..BUFFER_WIDTH {
            self.buffer.chars[row][i] = ScreenChar { ascii_character: 0, color_code: ColorCode(0) };
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
