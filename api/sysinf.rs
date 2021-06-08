use crate::devices::cpu;
use raw_cpuid::{VendorInfo};
/// Returns the CPU's vendor Information.
pub fn cpu_vendor() -> Option<VendorInfo> {
    cpu::vendor_name()
}

/// Returns the CPU's base frequency in Megahertz, or 0 if the frequency couldn't be attained.
pub fn cpu_base_frequency() -> u16 {
    cpu::frequency().unwrap_or_default().processor_base_frequency()
}