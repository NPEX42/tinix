//Copyright 2021, George Venn, GPL v3.0, NO WARRANTY

//We can't use the Rust Standard Library as a Baremetal Application
#![no_std]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]
// #![feature(alloc_error_handler)] // at the top of the file
#![feature(const_fn)]


use bootloader::BootInfo;

pub mod io;
pub mod qemu;
pub mod gfx;
pub mod interrupts;


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

