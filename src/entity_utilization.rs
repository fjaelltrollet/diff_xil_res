#[derive(Debug)]
pub struct EntityUtilization {
    lut: u32,
    reg: u32,
    dsp: u32,
    bram18: u32,
    uram: u32,
}