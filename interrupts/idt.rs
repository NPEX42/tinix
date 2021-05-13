use lazy_static::lazy_static;
use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};

use x86_64::structures::idt::PageFaultErrorCode;

use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

use x86_64::instructions::port::Port;
use crate::{println, print};
use crate::interrupts::pic::InterruptIndex;

lazy_static! {
    static ref IDT : InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint);
        unsafe {idt.double_fault.set_handler_fn(double_fault).set_stack_index(super::gdt::DOUBLE_FAULT_FIRST_INDEX);}

        idt.page_fault.set_handler_fn(page_fault_handler);

        idt[InterruptIndex::TIMER.as_usize()].set_handler_fn(timer_tick);
        idt[InterruptIndex::KEYBOARD.as_usize()].set_handler_fn(keyboard_interrupt);

        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault(_info : &mut InterruptStackFrame, _ec : u64) -> ! {
    panic!("Double Fault [{}]:\n{:#?}",_ec,_info);
}

extern "x86-interrupt" fn breakpoint(_info : &mut InterruptStackFrame) {
    println!("Breakpoint:\n{:#?}",_info);
}

extern "x86-interrupt" fn timer_tick(_info : &mut InterruptStackFrame) {
    print!(".");
    super::pic::fire_eoi(InterruptIndex::TIMER.as_u8());
}

extern "x86-interrupt" fn keyboard_interrupt(_info : &mut InterruptStackFrame) {
    let mut keyboard_port : Port<u8> = Port::new(0x60);
    let scancode = unsafe {keyboard_port.read()};

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
                HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("U[{}]", character),
                DecodedKey::RawKey(key) => print!("R[{:?}]", key),
            }
        }
    }
    super::pic::fire_eoi(InterruptIndex::KEYBOARD.as_u8());
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    loop {crate::pause(1)}
}


