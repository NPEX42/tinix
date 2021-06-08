use raw_cpuid::{CpuId, VendorInfo, ProcessorFrequencyInfo};

pub fn vendor_name() -> Option<VendorInfo> {
    CpuId::new().get_vendor_info()
}

pub fn frequency() -> Option<ProcessorFrequencyInfo> {
    CpuId::new().get_processor_frequency_info()
}