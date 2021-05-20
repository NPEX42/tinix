//Copyright 2021, George Venn, GPL v3.0, NO WARRANTY

//We can't use the Rust Standard Library as a Baremetal Application
#![no_std]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]
// #![feature(alloc_error_handler)] // at the top of the file
#![feature(const_fn)]
#![feature(asm)]

extern crate alloc;

use bootloader::BootInfo;

pub mod io;
pub mod qemu;
pub mod gfx;
pub mod interrupts;
pub mod utils;
pub mod devices;
pub mod maths;

pub fn set_tick_rate(rate : usize) {
    interrupts::pit::set_frequency(rate);
}

pub fn disable_interrupts() {
    x86_64::instructions::interrupts::disable();
}

pub fn enable_interrupts() {
    x86_64::instructions::interrupts::enable();
}


pub fn init_modules(_boot_info : &BootInfo) {
    interrupts::init();
}

pub fn init_modules_no_alloc() {
    interrupts::init();
}

pub fn breakpoint() {
    x86_64::instructions::interrupts::int3();
}

pub fn pause(ticks : usize) {
    for _ in 0..=ticks {
        x86_64::instructions::interrupts::enable_and_hlt();
    }
}

