use core::fmt::Write;
use crate::io::printer::Printer;
use x86_64::instructions::interrupts::{self, without_interrupts};
use crate::gfx::vga::{
    ScreenBuffer, ColorCode, Char, SCREEN_HEIGHT, SCREEN_WIDTH, Color, GLOBAL_VGA_BUFFER_2
};

use tinix_fs::api::{FileWriter, File, FileInteractor};

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Terminal> = Mutex::new(Terminal::new(ColorCode::from_colors(
        Color::White, Color::Blue
    )));
}

pub struct Terminal {
    row     : usize,
    col     : usize,
    color   : ColorCode,
    buffer  : &'static mut ScreenBuffer,
}

impl Terminal {
    fn clearrow(&mut self, row: usize) {
        for col in 0..SCREEN_WIDTH {
            self.buffer.set_char(col, row, Char::blank(self.color));
        }
    }

    fn new(color : ColorCode) -> Terminal {
        Terminal {
            row     : SCREEN_HEIGHT - 1,
            col     : 0,
            buffer  : ScreenBuffer::mono_text_mode80x25(),
            color   : color
        }
    }
}

impl Printer for Terminal {
    fn print_str(&mut self, s:&str) {
        for byte in s.bytes() {
            self.print_u8(byte);
        }
        crate::gfx::vga::swap_buffers();
    }

    fn print_u8(&mut self, b:u8) {
        if b == b'\n'               { self.newline(); return; }
        if self.col >= SCREEN_WIDTH { self.newline(); return; }
        if b == b'\t'               { self.tab();     return; }

        self.buffer.set_char(self.col, self.row, Char::new(b, self.color));
        self.col += 1;

    }

    fn newline(&mut self) {
        for row in 1..SCREEN_HEIGHT {
            for col in 0..SCREEN_WIDTH {
                let character = self.buffer.get_char(col, row);
                self.buffer.set_char(col,row - 1,character);
            }
        }
        self.clearrow(SCREEN_HEIGHT - 1);
        self.col = 0;
    }

    fn tab(&mut self) {
        for _ in 0..=4 {
            self.print_u8(b' ');
        }
    }


}

impl Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::terminal::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

pub fn get_char(x:usize, y:usize) -> Char {
    WRITER.lock().buffer.get_char(x,y)
}


pub struct StandardOut {_private : ()}

impl StandardOut {
    pub fn get() -> StandardOut {
        StandardOut {_private : ()}
    }
}

impl FileInteractor for StandardOut {
    fn at_end(&self) -> bool {
        false
    }

    fn close(_file : File) {
        
    }

    fn get_handle(_file : File) -> usize {
        2
    }
    fn get_pos(_file : File) -> usize {
        0
    }
    fn is_eof(_file : File) -> bool {
        false
    }
    fn set_pos(_pos  : usize, _file : File) {
        
    }

    fn open(_path : &str) -> File {
        File::new(2, 1)
    }
}

impl FileWriter<u8> for StandardOut {
    fn open(file : &File) -> Self {
        StandardOut {_private : ()}
    }

    fn write(&mut self, data : u8) {
        without_interrupts(|| {
            WRITER.lock().print_u8(data);
        })
    }

    fn flush(&mut self) {
        crate::gfx::swap();
    }
}
