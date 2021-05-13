use crate::interrupts::pic;
use x86_64::instructions::port::Port;
use crate::serial_println;

pub const PIT_FREQ : usize = 1193182; //1.193182MHz

pub fn set_frequency(freq : usize) {
    let mut mod_freq = freq;

    
    if(mod_freq < 18) { mod_freq = 18; }
    if(mod_freq > PIT_FREQ) {mod_freq = PIT_FREQ}
    serial_println!("Setting Frequency Value To {:}...", mod_freq);

    let reload_val : u16 = (mod_freq / PIT_FREQ) as u16;

    let HiByte : u8 = ((reload_val & 0xff00) >> 8) as u8;
    let LoByte : u8 = ((reload_val & 0x00ff) >> 0) as u8;

    let mut command_port : Port<u8> = Port::new(0x43);
    let mut data_port    : Port<u8> = Port::new(0x40);

    serial_println!("Setting Reload Value To {}...", reload_val);

    x86_64::instructions::interrupts::disable();
    let mut command : u8 = 0x36;
    serial_println!("Setting Reload Value To {:02x}...", command);

    unsafe {
        command_port.write(command);
        data_port.write(LoByte);
        data_port.write(HiByte);
    }
    x86_64::instructions::interrupts::enable();
}