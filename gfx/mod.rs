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

pub fn draw(x:usize, y:usize, chr:u8, fg:vga::Color, bg:vga::Color) {
    set_cell(x,y,chr,fg,bg)
}

pub fn draw_string(x:usize,y:usize, text:&str, color:(vga::Color, vga::Color)) {
    let mut mut_y : usize = y;
    let mut mut_x : usize = x;
    let mut mut_color = color;
    for chr in text.bytes() {
        if chr == b'\n' { 
            mut_y += 1;
            mut_x = x; 
        } else if is_color_escape(chr) {
            mut_color = escape_code_to_color_tuple(chr, mut_color);
        } else {
            draw(mut_x, mut_y, chr, mut_color.0, mut_color.1);
            mut_x += 1;
        }
        
    }
}

fn escape_code_to_color_tuple(chr:u8, active_colors:(vga::Color, vga::Color)) -> (vga::Color, vga::Color) {
    let mut mut_colors = active_colors;
    if chr >= 0x00 && chr <= 0x0f { //Set Background Color
        mut_colors.0 = vga::Color::from_u8(chr & 0x0f);
    }

    if chr >= 0x10 && chr <= 0x1f { //Set Foreground Color
        mut_colors.1 = vga::Color::from_u8(chr & 0x0f);
    }

    return mut_colors;
}

fn is_color_escape(chr:u8) -> bool {
    chr >= 0x00 && chr <= 0x1f
}

pub fn draw_rect(x:usize, y:usize, w:usize, h:usize, color:vga::Color) {
    for row in y..y+h {
        for col in x..x+w {
            draw(col, row, b' ', Color::Black, color);
        }
    }
}