#![allow(dead_code)]

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const VGA_GFX_MODE_START             : usize = 0xA0000;
const VGA_MONOCHROME_TEXT_MODE_START : usize = 0xB0000;
const VGA_COLOR_TEXT_MODE_START      : usize = 0xB8000;

pub const SCREEN_HEIGHT : usize = 25;
pub const SCREEN_WIDTH  : usize = 80;

lazy_static! {
    pub static ref GLOBAL_VGA_BUFFER : Mutex<&'static mut ScreenBuffer> = Mutex::new(
        ScreenBuffer::text_mode80x25()
    );
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Color {
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

impl Color {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(value : u8) -> Color {
        let mod_val = value % 16;
        match mod_val {
            0  => { Color::Black     }
            1  => { Color::Blue      }
            2  => { Color::Green     }
            3  => { Color::Cyan      }
            4  => { Color::Red       }
            5  => { Color::Magenta   } 
            6  => { Color::Brown     }
            7  => { Color::LightGray }
            8  => { Color::DarkGray  }
            9  => { Color::LightBlue }
            10 => { Color::LightGreen}
            11 => { Color::LightCyan }
            12 => { Color::LightRed  }
            13 => { Color::Pink      }
            14 => { Color::Yellow    } 
            15 => { Color::White     }
            _  => { Color::White     }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn from_u8(data : u8) -> ColorCode {
        ColorCode(data)
    }

    pub fn from_colors(fg : Color, bg : Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8)  & 0xf)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn bg_as_u8(&self) -> u8 {
        (self.as_u8() & 0xF0) >> 4
    }

    pub fn fg_as_u8(&self) -> u8 {
        self.as_u8() & 0x0F
    }

    pub fn bg_as_color(&self) -> Color {
        Color::from_u8(self.bg_as_u8())
    }

    pub fn fg_as_color(&self) -> Color {
        Color::from_u8(self.fg_as_u8())
    }

    pub fn set_fg_from_u8(&mut self, fg : u8) {
        self.0 = self.bg_as_u8() | fg
    } 

    pub fn set_bg_from_u8(&mut self, bg : u8) {
        self.0 = self.fg_as_u8() | bg
    } 
    
    pub fn set_bg_from_color(&mut self, bg : Color) {
        self.set_bg_from_u8(bg.as_u8())
    }

    pub fn set_fg_from_color(&mut self, fg : Color) {
        self.set_fg_from_u8(fg.as_u8())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Char {
    pub code_point : u8, 
    pub color : ColorCode
}

impl Char {
    pub fn new(ascii_char : u8, color : ColorCode) -> Char {
        Char {
            code_point: ascii_char,
            color: color
        }
    }

    pub fn blank(color : ColorCode) -> Char {
        Char::new(b' ', color)
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct ScreenBuffer {
    data : [[Volatile<Char> ; SCREEN_WIDTH] ; SCREEN_HEIGHT]
}

impl ScreenBuffer {
    pub fn text_mode80x25() -> &'static mut ScreenBuffer {
        unsafe { &mut *(VGA_COLOR_TEXT_MODE_START as *mut ScreenBuffer) }
    }

    pub fn set_char(&mut self, x:usize, y:usize, c:Char) {
        self.data[y][x].write(c);
    }

    pub fn get_char(&mut self, x:usize, y:usize) -> Char {
        self.data[y][x].read()
    }

    pub fn get_bg_as_color(self, x:usize, y:usize) -> Color {
        self.data[y][x].read().color.bg_as_color()
    }

    pub fn get_fg_as_color(self, x:usize, y:usize) -> Color {
        self.data[y][x].read().color.fg_as_color()
    }

    pub fn get_bg_as_u8(self, x:usize, y:usize) -> u8 {
        self.data[y][x].read().color.bg_as_u8()
    }

    pub fn get_fg_as_u8(&self, x:usize, y:usize) -> u8 {
        self.data[y][x].read().color.fg_as_u8()
    }

    pub fn get_ascii_char(&self, x:usize, y:usize) -> u8 {
        self.data[y][x].read().code_point
    }
}

