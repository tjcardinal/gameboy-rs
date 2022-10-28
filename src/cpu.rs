macro_rules! u16_register {
    ($getter:ident, $setter:ident, $hi:ident, $lo:ident) => {
        fn $getter(&self) -> u16 {
            (self.$hi as u16) << 8 | self.$lo as u16
        }

        fn $setter(&mut self, val: u16) {
            self.$hi = (val >> 8) as u8;
            self.$lo = (val & 0x00FF) as u8;
        }
    };
}

macro_rules! flag {
    ($getter:ident, $setter:ident, $index:literal) => {
        fn $getter(&self) -> bool {
            (self.f & 1 << $index) != 0
        }

        fn $setter(&mut self, val: bool) {
            self.f = match val {
                true => self.f | 1 << $index,
                false => self.f & (0xFF ^ (1 << $index)),
            }
        }
    };
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    u16_register!(af, set_af, a, f);
    u16_register!(bc, set_bc, b, c);
    u16_register!(de, set_de, d, e);
    u16_register!(hl, set_hl, h, l);

    flag!(zero_flag, set_zero_flag, 7);
    flag!(subtraction_flag, set_subtraction_flag, 6);
    flag!(half_carry_flag, set_half_carry_flag, 5);
    flag!(carry_flag, set_carry_flag, 4);
}

pub struct Cpu {
    registers: Registers,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn af_gets_a_f() {
        let registers = Registers {
            a: 0xAB,
            f: 0xCD,
            ..Default::default()
        };
        assert_eq!(registers.af(), 0xABCD);
    }

    #[test]
    fn bc_gets_b_c() {
        let registers = Registers {
            b: 0xAB,
            c: 0xCD,
            ..Default::default()
        };
        assert_eq!(registers.bc(), 0xABCD);
    }

    #[test]
    fn de_gets_d_e() {
        let registers = Registers {
            d: 0xAB,
            e: 0xCD,
            ..Default::default()
        };
        assert_eq!(registers.de(), 0xABCD);
    }

    #[test]
    fn hl_gets_h_l() {
        let registers = Registers {
            h: 0xAB,
            l: 0xCD,
            ..Default::default()
        };
        assert_eq!(registers.hl(), 0xABCD);
    }

    #[test]
    fn set_af_sets_a_f() {
        let mut registers = Registers::default();
        registers.set_af(0xABCD);
        assert_eq!(registers.a, 0xAB);
        assert_eq!(registers.f, 0xCD);
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

    #[test]
    fn zero_flag_gets_bit_7() {
        let registers = Registers {
            f: 0b10000000,
            ..Default::default()
        };
        assert_eq!(registers.zero_flag(), true);

        let registers = Registers {
            f: 0b01111111,
            ..Default::default()
        };
        assert_eq!(registers.zero_flag(), false);
    }

    #[test]
    fn subtraction_flag_gets_bit_6() {
        let registers = Registers {
            f: 0b01000000,
            ..Default::default()
        };
        assert_eq!(registers.subtraction_flag(), true);

        let registers = Registers {
            f: 0b10111111,
            ..Default::default()
        };
        assert_eq!(registers.subtraction_flag(), false);
    }

    #[test]
    fn half_carry_flag_gets_bit_5() {
        let registers = Registers {
            f: 0b00100000,
            ..Default::default()
        };
        assert_eq!(registers.half_carry_flag(), true);

        let registers = Registers {
            f: 0b11011111,
            ..Default::default()
        };
        assert_eq!(registers.half_carry_flag(), false);
    }

    #[test]
    fn carry_flag_gets_bit_4() {
        let registers = Registers {
            f: 0b00010000,
            ..Default::default()
        };
        assert_eq!(registers.carry_flag(), true);

        let registers = Registers {
            f: 0b11101111,
            ..Default::default()
        };
        assert_eq!(registers.carry_flag(), false);
    }

    #[test]
    fn set_zero_flag_sets_bit_7() {
        let mut registers = Registers::default();
        registers.set_zero_flag(true);
        assert_eq!(registers.f, 0b10000000);
        registers.set_zero_flag(false);
        assert_eq!(registers.f, 0);
    }

    #[test]
    fn set_subtraction_flag_sets_bit_6() {
        let mut registers = Registers::default();
        registers.set_subtraction_flag(true);
        assert_eq!(registers.f, 0b01000000);
        registers.set_subtraction_flag(false);
        assert_eq!(registers.f, 0);
    }

    #[test]
    fn set_half_carry_flag_sets_bit_5() {
        let mut registers = Registers::default();
        registers.set_half_carry_flag(true);
        assert_eq!(registers.f, 0b00100000);
        registers.set_half_carry_flag(false);
        assert_eq!(registers.f, 0);
    }

    #[test]
    fn set_carry_flag_sets_bit_4() {
        let mut registers = Registers::default();
        registers.set_carry_flag(true);
        assert_eq!(registers.f, 0b00010000);
        registers.set_carry_flag(false);
        assert_eq!(registers.f, 0);
    }
}
