use x86_64::instructions::port::*;

type RegisterRW = Port<u8>;
type RegisterWO = PortWriteOnly<u8>;
type RegisterRO = PortReadOnly<u8>;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum VideoMode {
    TextMode8025    = 0x03,
    ColorMode16     = 0x12,
    LinearColor256  = 0x13,
    // TODO: Add Support For 256-Color Planar Mode (Mode X) 
}

pub struct Vga {
    index_data          : RegisterRW,
    misc_output_read    : RegisterRO,
    mise_output_write   : RegisterWO,
    dac_mask            : RegisterRW,
    color_index         : RegisterRW,
    color_data          : RegisterRW,
}

pub(crate) struct CrtController {
    horizontal_total_reg    : RegisterRW,
    end_horizontal_disp_reg : RegisterRW, 
} 