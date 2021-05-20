use super::vga::Color;
use super::drawables::Drawable;

#[derive(Clone, Copy, Debug)]
pub struct ProgressBar {
    fill        : f32,
    min         : usize,
    max         : usize,
    value       : usize,
    scale       : usize,
    x_pos       : usize,
    y_pos       : usize,
    base_col    : Color
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
        super::draw_string!(self.x_pos + self.scale, self.y_pos, (Color::White, Color::Blue), " | {:03.02}%", (self.fill * 100f32));
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
            value       :   min
        }
    }

    pub fn set_value(&mut self, value : usize) {
        self.value = value;
        self.update();
    }

    fn update(&mut self) {
        self.fill = crate::maths::map01_f(self.value as f32, self.min as f32, self.max as f32)
    }
}