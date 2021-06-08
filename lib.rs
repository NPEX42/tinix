//Copyright 2021, George Venn, GPL v3.0, NO WARRANTY

//We can't use the Rust Standard Library as a Baremetal Application
#![no_std]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]
// #![feature(alloc_error_handler)] // at the top of the file
#![feature(const_fn)]
#![feature(asm)]

#![allow(non_camel_case_types)]
#![allow(dead_code, deprecated)]
#![allow(unused_assignments)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::BootInfo;
use tinix_fs::api::{FileReader, FileWriter};

pub mod io;
pub mod qemu;
pub mod gfx;
pub mod interrupts;
pub mod utils;
pub mod devices;
pub mod maths;
pub mod api;
mod tests;

pub use api as user; 

pub use alloc::boxed::Box as Box;
pub use alloc::string::String as String;
pub use alloc::vec as vec;

pub use alloc::vec::Vec as Vec;

static mut FREQ : usize = 0;

pub fn set_tick_rate(rate : usize) {
    interrupts::pit::set_frequency(rate);
    unsafe {FREQ = rate};
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

pub fn pause_seconds(seconds : f32) {
    pause((seconds * get_frequency() as f32) as usize)
}

pub fn get_frequency() -> usize {
    unsafe {FREQ}
}

pub fn stdin() -> impl FileReader<char> {
    crate::devices::keyboard::StandardIn::get()
}

pub fn stdout() -> impl FileWriter<u8> {
    crate::io::terminal::StandardOut::get()
}

