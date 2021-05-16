pub fn get_al() -> i8 {
    unsafe {
        let mut value : i8 = 0;
        asm!("mov {}, al", out(reg_byte) value);
        return value
    }
}

pub fn get_ah() -> i8 {
    unsafe {
        let mut value : i8 = 0;
        asm!("mov {}, ah", out(reg_byte) value);
        return value
    }
}

pub fn get_ax() -> i16 {
    return (((get_ah() as i16) << 8) | get_al() as i16) as i16;
}
