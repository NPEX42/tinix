use core::fmt::{Display, Formatter};

use crate::devices::cmos::CMOS;
/// Holds a date
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8
}

/// Holds a Time, accurate to the second
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

///Wraps Together a Date & Time
pub struct DateTime {
    date : Date,
    time : Time
}

/// Get the current time from the Real-Time Clock
pub fn time() -> Time {
    let rtc = CMOS::new().rtc();
    Time {
        second  : rtc.second,
        minute  : rtc.minute,
        hour    : rtc.hour,
    }
}

/// Get the current Date from the Real-Time Clock
pub fn date() -> Date {
    let rtc = CMOS::new().rtc();
    Date {
        day     : rtc.day,
        month   : rtc.month,
        year    : rtc.year,
    }
}

/// Get the current Date & Time from the Real-Time Clock
pub fn date_time() -> DateTime {
    DateTime {date : date(), time : time()}
}


const DAYS_BEFORE_MONTH: [u64; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];

/// Get the uptime of the kernal in Seconds
pub fn uptime() -> f64 {
    (crate::interrupts::global_timer::current_tick() as f64) / crate::get_frequency() as f64
}

// NOTE: This clock is not monotonic
pub fn realtime() -> f64 {
    let rtc = CMOS::new().rtc(); // Assuming GMT

    let timestamp = 86400 * days_before_year(rtc.year as u64)
                  + 86400 * days_before_month(rtc.year as u64, rtc.month as u64)
                  + 86400 * (rtc.day - 1) as u64
                  +  3600 * rtc.hour as u64
                  +    60 * rtc.minute as u64
                  +         rtc.second as u64;

    let fract = 0f64;

    (timestamp as f64) + fract
}




fn days_before_year(year: u64) -> u64 {
    (1970..year).fold(0, |days, y| {
        days + if is_leap_year(y) { 366 } else { 365 }
    })
}

fn days_before_month(year: u64, month: u64) -> u64 {
    let leap_day = is_leap_year(year) && month > 2;
    DAYS_BEFORE_MONTH[(month as usize) - 1] + if leap_day { 1 } else { 0 }
}

fn is_leap_year(year: u64) -> bool {
    if year % 4 != 0 {
        false
    } else if year % 100 != 0 {
        true
    } else if year % 400 != 0 {
        false
    } else {
        true
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02}/{:02}/{:04}",self.day, self.month, self.year)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}",self.hour, self.minute, self.second)
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} - {}", self.date, self.time)
    }
}