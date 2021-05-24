use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts::without_interrupts;
struct Timer(u128);

lazy_static! {
    static ref GLOBAL_TIMER : Mutex<Timer> = Mutex::new(Timer(0));
}

pub(crate) fn update() {
    without_interrupts(|| {GLOBAL_TIMER.lock().0 += 1});
}

pub(crate) fn reset() {
    without_interrupts(|| {GLOBAL_TIMER.lock().0 = 0});
}

pub fn current_tick() -> u128 {
    let mut x : u128 = 0;
    without_interrupts(|| {x = GLOBAL_TIMER.lock().0});
    x
}

pub fn get_seconds() -> f64 {
    (current_tick() as f64) / (crate::get_frequency() as f64)
}

pub fn get_minutes() -> f64 {
    get_seconds() / 60f64
}