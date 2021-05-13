pub mod vga;

use x86_64::instructions::interrupts::without_interrupts;

use vga::Char;

use vga::{Color, ColorCode};

pub fn set_cell_color(x:usize, y:usize, fg:vga::Color, bg:vga::Color) {
    without_interrupts(|| {
        let char = vga::GLOBAL_VGA_BUFFER.lock().get_ascii_char(x, y);
        vga::GLOBAL_VGA_BUFFER.lock().set_char(
                x, y,
            Char::new(char,ColorCode::from_colors(fg, bg)
            )
        );
    });
}

pub fn set_cell(x:usize, y:usize, chr:u8, fg:vga::Color, bg:vga::Color) {
    without_interrupts(|| {
        vga::GLOBAL_VGA_BUFFER.lock().set_char(
                x, y,
            Char::new(chr,ColorCode::from_colors(fg, bg)
            )
        );
    });
}

pub fn get_bg(x:usize, y:usize) -> Color {
    vga::GLOBAL_VGA_BUFFER.lock().get_char(x,y).color.bg_as_color()
}

pub fn clear(bg : Color) {
    for y in 0..vga::SCREEN_HEIGHT {
        for x in 0..vga::SCREEN_WIDTH {
            set_cell(x, y,b' ', Color::White, bg);
        }
    }
}