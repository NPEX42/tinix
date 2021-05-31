use super::vga::Color;
use super::drawables::Drawable;
use alloc::{
    vec,
    vec::Vec,
    string::String
};

use crate::maths::*;
#[derive(Clone, Copy, Debug)]
pub struct ProgressBar {
    fill        : f32,
    min         : usize,
    max         : usize,
    value       : usize,
    scale       : usize,
    x_pos       : usize,
    y_pos       : usize,
    base_col    : Color,
    text_col    : (Color, Color)
}

impl Drawable for ProgressBar {
    fn draw_self(&self) {
        super::draw_rect(
            self.x_pos,
            self.y_pos, 
            (self.fill * (self.scale as f32)) as usize,
            1,
            self.base_col
        );
        super::draw_string!(self.x_pos + self.scale, self.y_pos, self.text_col, " | {:03.02}%", (self.fill * 100f32));
    }
}

impl ProgressBar {
    pub fn new(x:usize, y:usize, color:super::vga::Color, min:usize, max:usize, scale:usize) -> ProgressBar {
        ProgressBar {
            x_pos       :   x,
            y_pos       :   y,
            base_col    :   color,
            min         :   min,
            max         :   max,
            scale       :   scale,

            fill        :   0f32,
            value       :   min,

            text_col    :   (Color::White, Color::Blue)
        }
    }

    pub fn set_value(&mut self, value : usize) {
        self.value = value;
        self.update();
    }

    pub fn set_text_color(&mut self, col : (Color, Color)) {
        self.text_col = col
    }

    fn update(&mut self) {
        self.fill = crate::maths::map01_f(self.value as f32, self.min as f32, self.max as f32)
    }


}



pub struct TextArea {
    x           : usize,
    y           : usize,
    idx         : usize,
    max_lines   : usize,
    max_cols    : usize,
    text        : Vec<String>,
    color       : (Color, Color),
    use_line_no : bool,
}

impl TextArea {
    pub fn new(x : usize, y : usize, max_lines : usize, max_cols : usize, color : (Color, Color)) -> TextArea {
        TextArea {
            x           : x,
            y           : y,
            idx         : 0,
            max_lines   : max_lines,
            max_cols    : max_cols,
            text        : Vec::new(),
            color       : color,
            use_line_no : true
        }
    }

    pub fn set_index(&mut self, idx : usize) {
        self.idx = clamp_us(idx, 0, self.max_lines);
    }

    pub fn remove_index(&mut self, idx : usize) {
        self.text.remove(idx);
    }

    pub fn append_line(&mut self, line : String) {
        self.text.push(line);
    }

    pub fn scroll(&mut self, dx : isize) { 
        self.set_index(((self.idx as isize) + dx) as usize);
    }

    pub fn size(&mut self) -> usize {
        self.text.len()
    }

    pub fn enable_line_nos(&mut self) {
        self.use_line_no = true;
    }

    pub fn disable_line_nos(&mut self) {
        self.use_line_no = false;
    }
}

impl Drawable for TextArea {
    fn draw_self(&self) {
        let mut idx = 1;
        let mut x = 1;
        if self.text.len() > self.max_lines { idx += self.text.len() - self.max_lines }
        for i in idx..=self.text.len() {
        let line = self.text.get(i).unwrap();
        if self.use_line_no {
            super::draw_string!(0,x,(Color::White, Color::Blue), "[{:02}] {}",idx, line);
        } else {
            super::draw_string!(0,x,(Color::White, Color::Blue), "{}", line);
        }
        idx += 1;
        x += 1;
        }   
    }
}