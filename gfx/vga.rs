#![allow(dead_code)]

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use x86_64::instructions::port::Port;
use x86_64::instructions::interrupts::without_interrupts;
use core::borrow::Borrow;

const VGA_BUFFERS_START     : usize = 0x80000;
const VGA_BUFFER_1_START    : usize = 0x90000;
const VGA_BUFFER_2_START    : usize = 0x98000;      

const VGA_GFX_MODE_START             : usize = 0xA0000;
const VGA_MONOCHROME_TEXT_MODE_START : usize = 0xB0000;
const VGA_COLOR_TEXT_MODE_START      : usize = 0xB8000;

pub const SCREEN_HEIGHT : usize = 25;
pub const SCREEN_WIDTH  : usize = 80;

pub const GFX_SCREEN_HEIGHT : usize = 200;
pub const GFX_SCREEN_WIDTH  : usize = 320;

lazy_static! {
    pub static ref GLOBAL_VGA_BUFFER : Mutex<&'static mut ScreenBuffer> = Mutex::new(
        ScreenBuffer::text_mode80x25()
    );
}

lazy_static! {
    pub static ref GLOBAL_VGA_BUFFER_2 : Mutex<&'static mut ScreenBuffer> = Mutex::new(
        ScreenBuffer::from_addr(0x10000)
    );
}

lazy_static! {
    pub static ref GLOBAL_VGA_BUFFER_3 : Mutex<&'static mut ScreenBuffer> = Mutex::new(
        ScreenBuffer::gfx_l32k()
    );
}

lazy_static! {
    pub static ref GLOBAL_GFX_BUFFER : Mutex<&'static mut GraphicsBuffer> = Mutex::new(
        GraphicsBuffer::new()
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

pub fn swap_buffers() {
    without_interrupts(|| {
        //crate::serial_println!("Swapping Buffers...");
    //Preserve COLOR_VGA_TM Buffer in Low 32K of the GFX Buffer
    {
        GLOBAL_VGA_BUFFER.lock().copy_to(VGA_GFX_MODE_START);
    }
    //Copy Monochrome Buffer to Color Buffer
    {
        GLOBAL_VGA_BUFFER_2.lock().copy_to(VGA_COLOR_TEXT_MODE_START);
    }
    //Copy Low 32K of the GFX Buffer to the Monochrome Buffer
    {
        GLOBAL_VGA_BUFFER_3.lock().copy_to(VGA_MONOCHROME_TEXT_MODE_START)
    }
    });
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
        ScreenBuffer::from_addr(VGA_COLOR_TEXT_MODE_START)
    }

    pub fn mono_text_mode80x25() -> &'static mut ScreenBuffer {
        ScreenBuffer::from_addr(VGA_MONOCHROME_TEXT_MODE_START)
    }

    pub fn gfx_l32k() -> &'static mut ScreenBuffer {
        ScreenBuffer::from_addr(VGA_GFX_MODE_START)
    }

    pub fn gfx_h32k() -> &'static mut ScreenBuffer {
        ScreenBuffer::from_addr(VGA_GFX_MODE_START + 32768)
    }

    pub fn from_addr(start : usize) -> &'static mut ScreenBuffer {
        unsafe { &mut *(start as *mut ScreenBuffer) }
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

    pub fn copy_to(&self,addr : usize) {
        let buffer = ScreenBuffer::from_addr(addr);
        for y in 0..25 {
            for x in 0..80 {
                buffer.data[y][x].write(self.data[y][x].read());
            };
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum VgaMode {
    TEXT_80x25  = 0x03,
    GFX_320x200 = 0x13
}

impl VgaMode {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

const VGA_INDEX_DATA_PORT  : u16 = 0x3C0;
const VGA_MODE_INDEX       : u8  = 0x010;
const VGA_INDEX_RESET_PORT : u16 = 0x3DA;

pub fn set_mode(mode : VgaMode) {
    write_to_data_port(VGA_MODE_INDEX, mode.as_u8())
}

fn reset_vga_index_data_port() {
    let mut port : Port<u8> = Port::new(VGA_INDEX_RESET_PORT);
    unsafe {port.read();}
}

fn write_to_data_port(index : u8, data : u8) {
    write_index(index);
    write_data(data);
}

fn write_index(index : u8) {
    reset_vga_index_data_port();
    let mut port : Port<u8> = Port::new(VGA_INDEX_DATA_PORT);
    unsafe {port.write(index)}
}

fn write_data(data : u8) {
    let mut port : Port<u8> = Port::new(VGA_INDEX_RESET_PORT);
    unsafe {port.write(data)}
}






//Graphics Mode Stuff ======================================================================

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Pixel(u8);

impl Pixel {
    pub fn as_u8(self) -> u8 {
        self.0
    }

    pub fn from_u8(c : u8) -> Pixel {
        Pixel(c)
    }

    pub fn from_color(c : Color) -> Pixel {
        Pixel::from_u8(c.as_u8())
    }

    pub fn new(c : u8) -> Pixel {
        Pixel(c)
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct GraphicsBuffer {
    pixels : [[Volatile<Pixel> ; 320]; 200]
}

impl GraphicsBuffer {
    pub fn new() -> &'static mut GraphicsBuffer {
        unsafe { &mut *(VGA_GFX_MODE_START as *mut GraphicsBuffer) }
    }

    pub fn set_pixel(&mut self, x:usize, y:usize, pixel:Pixel) {
        self.pixels[y][x].write(pixel)
    }

    pub fn get_pixel(&mut self, x:usize, y:usize) -> Pixel {
        self.pixels[y][x].read()
    }

    pub fn fill(&mut self, pixel:Pixel) {
        for x in 0..GFX_SCREEN_WIDTH {
            for y in 0..GFX_SCREEN_HEIGHT {
                self.set_pixel(x,y, pixel);
            }  
        }
    }
}





