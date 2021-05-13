use pic8259_simple::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;


pub static PICS : Mutex<ChainedPics> = Mutex::new(
    unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)}
);



pub fn init() {
    unsafe {
        PICS.lock().initialize();
    }
}
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum InterruptIndex {
    TIMER = PIC_1_OFFSET,
    KEYBOARD
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        self as usize
    }
}


pub fn fire_eoi(id : u8) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(id);
    }
}