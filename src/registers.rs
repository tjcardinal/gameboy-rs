use crate::flags::Flags;

pub enum U8Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
pub enum U16Register {
    BC,
    DE,
    HL,
    SP,
}

macro_rules! u16_register {
    ($getter:ident, $setter:ident, $hi:ident, $lo:ident) => {
        pub fn $getter(&self) -> u16 {
            (self.$hi as u16) << 8 | u8::from(self.$lo) as u16
        }

        pub fn $setter(&mut self, val: u16) {
            self.$hi = (val >> 8) as u8;
            self.$lo = ((val & 0x00FF) as u8).into();
        }
    };
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    u16_register!(af, set_af, a, f);
    u16_register!(bc, set_bc, b, c);
    u16_register!(de, set_de, d, e);
    u16_register!(hl, set_hl, h, l);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn af_gets_a_f() {
        let registers = Registers {
            a: 0xAB,
            f: 0xF0.into(),
            ..Registers::default()
        };
        assert_eq!(registers.af(), 0xABF0);
    }

    #[test]
    fn bc_gets_b_c() {
        let registers = Registers {
            b: 0xAB,
            c: 0xCD,
            ..Registers::default()
        };
        assert_eq!(registers.bc(), 0xABCD);
    }

    #[test]
    fn de_gets_d_e() {
        let registers = Registers {
            d: 0xAB,
            e: 0xCD,
            ..Registers::default()
        };
        assert_eq!(registers.de(), 0xABCD);
    }

    #[test]
    fn hl_gets_h_l() {
        let registers = Registers {
            h: 0xAB,
            l: 0xCD,
            ..Registers::default()
        };
        assert_eq!(registers.hl(), 0xABCD);
    }

    #[test]
    fn set_af_sets_a_f() {
        let mut registers = Registers::default();
        registers.set_af(0xABCD);
        assert_eq!(registers.a, 0xAB);
        assert_eq!(registers.f, 0xCD.into());
    }

    #[test]
    fn set_bc_sets_b_c() {
        let mut registers = Registers::default();
        registers.set_bc(0xABCD);
        assert_eq!(registers.b, 0xAB);
        assert_eq!(registers.c, 0xCD);
    }

    #[test]
    fn set_de_sets_d_e() {
        let mut registers = Registers::default();
        registers.set_de(0xABCD);
        assert_eq!(registers.d, 0xAB);
        assert_eq!(registers.e, 0xCD);
    }

    #[test]
    fn set_hl_sets_h_l() {
        let mut registers = Registers::default();
        registers.set_hl(0xABCD);
        assert_eq!(registers.h, 0xAB);
        assert_eq!(registers.l, 0xCD);
    }
}
