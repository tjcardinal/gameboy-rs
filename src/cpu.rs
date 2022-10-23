#[derive(Debug, Default, PartialEq, Eq)]
pub struct Register(u16);

impl Register {
    fn read_hi(&self) -> u16 {
        self.0 >> 8
    }

    fn write_hi(&mut self, val: u16) {
        self.0 = (val << 8) | self.read_lo();
    }

    fn read_lo(&self) -> u16 {
        self.0 & 0xFF
    }

    fn write_lo(&mut self, val: u16) {
        self.0 = (self.read_hi() << 8) | val
    }

    fn read_bit(&self, index: usize) -> bool {
        (self.0 & 1 << index) != 0
    }

    fn write_bit(&mut self, index: usize, val: bool) {
        self.0 = match val {
            true => self.0 | 1 << index,
            false => self.0 & (0xFFFF ^ (1 << index)),
        }
    }
}

pub struct Cpu {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: Register,
    pc: Register,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_hi() {
        let reg = Register(0xABCD);
        assert_eq!(reg.read_hi(), 0xAB);
    }

    #[test]
    fn write_hi() {
        let mut reg = Register(0xF0CD);
        reg.write_hi(0xAB);
        assert_eq!(reg.0, 0xABCD);
    }

    #[test]
    fn read_lo() {
        let reg = Register(0xABCD);
        assert_eq!(reg.read_lo(), 0xCD);
    }

    #[test]
    fn write_lo() {
        let mut reg = Register(0xAB0F);
        reg.write_lo(0xCD);
        assert_eq!(reg.0, 0xABCD);
    }

    #[test]
    fn read_bits() {
        let reg = Register(0xABCD);

        // A = 1010
        assert_eq!(reg.read_bit(15), true);
        assert_eq!(reg.read_bit(14), false);
        assert_eq!(reg.read_bit(13), true);
        assert_eq!(reg.read_bit(12), false);

        // B = 1011
        assert_eq!(reg.read_bit(11), true);
        assert_eq!(reg.read_bit(10), false);
        assert_eq!(reg.read_bit(9), true);
        assert_eq!(reg.read_bit(8), true);

        // C = 1100
        assert_eq!(reg.read_bit(7), true);
        assert_eq!(reg.read_bit(6), true);
        assert_eq!(reg.read_bit(5), false);
        assert_eq!(reg.read_bit(4), false);

        // D = 1101
        assert_eq!(reg.read_bit(3), true);
        assert_eq!(reg.read_bit(2), true);
        assert_eq!(reg.read_bit(1), false);
        assert_eq!(reg.read_bit(0), true);
    }

    #[test]
    fn set_bits() {
        let mut reg = Register(0x0000);

        for index in 0..16 {
            reg.write_bit(index, true);
        }
        assert_eq!(reg.0, 0xFFFF);
    }

    #[test]
    fn unset_bits() {
        let mut reg = Register(0xFFFF);

        for index in 0..16 {
            reg.write_bit(index, false);
        }
        assert_eq!(reg.0, 0x0000);
    }
}
