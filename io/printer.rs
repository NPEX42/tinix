use core::fmt::Write;

pub trait Printer : Write {
    fn print_str(&mut self, s : &str);
    fn print_u8 (&mut self, b : u8);
    fn newline(&mut self);
    fn tab(&mut self);
}
