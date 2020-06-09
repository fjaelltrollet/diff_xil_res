use std::ops;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EntityUtilization {
    pub lut: i32,
    pub reg: i32,
    pub dsp: i32,
    pub bram18: i32,
    pub uram: i32,
}

impl<'a> ops::Neg for &'a EntityUtilization {
    type Output = EntityUtilization;

    fn neg(self) -> EntityUtilization {
        EntityUtilization {
            lut: -self.lut,
            reg: -self.reg,
            dsp: -self.dsp,
            bram18: -self.bram18,
            uram: -self.uram
        }
    }
}

impl<'a, 'b> ops::Add<&'b EntityUtilization> for &'a EntityUtilization {
    type Output = EntityUtilization;

    fn add(self, other: &'b EntityUtilization) -> EntityUtilization {
        EntityUtilization {
            lut: self.lut + other.lut,
            reg: self.reg + other.reg,
            dsp: self.dsp + other.dsp,
            bram18: self.bram18 + other.bram18,
            uram: self.uram + other.uram,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b EntityUtilization> for &'a EntityUtilization {
    type Output = EntityUtilization;

    fn sub(self, other: &'b EntityUtilization) -> EntityUtilization {
        EntityUtilization {
            lut: self.lut - other.lut,
            reg: self.reg - other.reg,
            dsp: self.dsp - other.dsp,
            bram18: self.bram18 - other.bram18,
            uram: self.uram - other.uram,
        }
    }
}
