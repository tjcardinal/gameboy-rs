use crate::registers::{Registers, U16Register, U8Register};

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    // 8-bit load
    fn ld(&mut self, id: U8Register, val: u8) {
        let reg = match id {
            U8Register::A => &mut self.registers.a,
            U8Register::B => &mut self.registers.b,
            U8Register::C => &mut self.registers.c,
            U8Register::D => &mut self.registers.d,
            U8Register::E => &mut self.registers.e,
            U8Register::H => &mut self.registers.h,
            U8Register::L => &mut self.registers.l,
        };
        *reg = val;
    }

    fn ldi(&mut self, id: U8Register, val: u8) {
        let reg = match id {
            U8Register::A => &mut self.registers.a,
            U8Register::B => &mut self.registers.b,
            U8Register::C => &mut self.registers.c,
            U8Register::D => &mut self.registers.d,
            U8Register::E => &mut self.registers.e,
            U8Register::H => &mut self.registers.h,
            U8Register::L => &mut self.registers.l,
        };
        *reg = val;
        self.registers.set_hl(self.registers.hl() + 1);
    }

    fn ldd(&mut self, id: U8Register, val: u8) {
        let reg = match id {
            U8Register::A => &mut self.registers.a,
            U8Register::B => &mut self.registers.b,
            U8Register::C => &mut self.registers.c,
            U8Register::D => &mut self.registers.d,
            U8Register::E => &mut self.registers.e,
            U8Register::H => &mut self.registers.h,
            U8Register::L => &mut self.registers.l,
        };
        *reg = val;
        self.registers.set_hl(self.registers.hl() + 1);
    }

    // 16-bit load
    fn ld_u16(&mut self, id: U16Register, val: u16) {
        let reg = match id {
            U16Register::BC => self.registers.set_bc(val),
            U16Register::DE => self.registers.set_de(val),
            U16Register::HL => self.registers.set_hl(val),
            U16Register::SP => self.registers.sp = val,
        };
    }






    // 8-bit arithmetic/Logic
    fn add(&mut self, val: u8) {
        let (new_value, overflowed) = self.registers.a.overflowing_add(val);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = (self.registers.a & 0x0F) + (val & 0x0F) > 0x0F;
        self.registers.f.carry = overflowed;
        self.registers.a = new_value;
    }

    fn adc(&mut self, val: u8) {
        let (first_value, first_overflowed) = self.registers.a.overflowing_add(val);
        let (second_value, second_overflowed) =
            first_value.overflowing_add(self.registers.f.carry as u8);
        self.registers.f.zero = second_value == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry =
            (self.registers.a & 0x0F) + (val & 0x0F) + (self.registers.f.carry as u8 & 0x0F) > 0x0F;
        self.registers.f.carry = first_overflowed | second_overflowed;
        self.registers.a = second_value;
    }

    fn sub(&mut self, val: u8) {
        let (new_value, overflowed) = self.registers.a.overflowing_sub(val);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = true;
        self.registers.f.half_carry = (self.registers.a & 0x0F) - (val & 0x0F) > 0x0F;
        self.registers.f.carry = overflowed;
        self.registers.a = new_value;
    }

    fn sbc(&mut self, val: u8) {
        let (first_value, first_overflowed) = self.registers.a.overflowing_add(val);
        let (second_value, second_overflowed) =
            first_value.overflowing_add(self.registers.f.carry as u8);
        self.registers.f.zero = second_value == 0;
        self.registers.f.subtraction = true;
        self.registers.f.half_carry =
            (self.registers.a & 0x0F) - (val & 0x0F) - (self.registers.f.carry as u8 & 0x0F) > 0x0F;
        self.registers.f.carry = first_overflowed | second_overflowed;
        self.registers.a = second_value;
    }

    fn and(&mut self, val: u8) {
        self.registers.a = self.registers.a & val;
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
    }

    fn xor(&mut self, val: u8) {
        self.registers.a = self.registers.a ^ val;
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }

    fn or(&mut self, val: u8) {
        self.registers.a = self.registers.a | val;
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }

    fn cp(&mut self, val: u8) {
        let (new_value, overflowed) = self.registers.a.overflowing_sub(val);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = true;
        self.registers.f.half_carry = (self.registers.a & 0x0F) - (val & 0x0F) > 0x0F;
        self.registers.f.carry = overflowed;
    }

    fn inc(&mut self, id: U8Register) {
        let reg = match id {
            U8Register::A => &mut self.registers.a,
            U8Register::B => &mut self.registers.b,
            U8Register::C => &mut self.registers.c,
            U8Register::D => &mut self.registers.d,
            U8Register::E => &mut self.registers.e,
            U8Register::H => &mut self.registers.h,
            U8Register::L => &mut self.registers.l,
        };
        let new_value = reg.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = (*reg & 0x0F) + 1 > 0x0F;
        *reg = new_value;
    }

    fn dec(&mut self, id: U8Register) {
        let reg = match id {
            U8Register::A => &mut self.registers.a,
            U8Register::B => &mut self.registers.b,
            U8Register::C => &mut self.registers.c,
            U8Register::D => &mut self.registers.d,
            U8Register::E => &mut self.registers.e,
            U8Register::H => &mut self.registers.h,
            U8Register::L => &mut self.registers.l,
        };
        let new_value = reg.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = true;
        self.registers.f.half_carry = (*reg & 0x0F) - 1 > 0x0F;
        *reg = new_value;
    }

    fn daa(&mut self) {
        if self.registers.f.subtraction {
            self.registers.a = self.registers.a -
                (self.registers.f.carry || self.registers.a > 0x9F) as u8 * 0x60 -
                (self.registers.f.half_carry || self.registers.a & 0x0F > 0x09) as u8 * 0x06;

        } else {
            self.registers.a = self.registers.a +
                (self.registers.f.carry || self.registers.a > 0x9F) as u8 * 0x60 +
                (self.registers.f.half_carry || self.registers.a & 0x0F > 0x09) as u8 * 0x06;
        }
        self.registers.f.carry = self.registers.a > 0x99;
        self.registers.f.zero = self.registers.a == 0;
    }

    fn cpl(&mut self) {
        self.registers.a = self.registers.a ^ 0xFF;
        self.registers.f.subtraction = true;
        self.registers.f.half_carry = true;
    }

    // 16-bit arithmetic/Logic
    fn add_HL(&mut self, id: U16Register) {
        let reg = match id {
            U16Register::BC => self.registers.bc(),
            U16Register::DE => self.registers.de(),
            U16Register::HL => self.registers.hl(),
            U16Register::SP => self.registers.sp,
        };
        let (new_value, overflowed) = self.registers.hl().overflowing_add(reg);
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = (self.registers.hl() & 0x0F00) + (reg & 0x0F00) > 0x0F00;
        self.registers.f.carry = overflowed;
        self.registers.set_hl(new_value);
    }

    fn inc_u16(&mut self, id: U16Register) {
        match id {
            U16Register::BC => {
                self.registers.set_bc(self.registers.bc().wrapping_add(1));
            }
            U16Register::DE => {
                self.registers.set_de(self.registers.de().wrapping_add(1));
            }
            U16Register::HL => {
                self.registers.set_hl(self.registers.hl().wrapping_add(1));
            }
            U16Register::SP => {
                self.registers.sp = self.registers.sp.wrapping_add(1);
            }
        }
    }

    fn dec_u16(&mut self, id: U16Register) {
        match id {
            U16Register::BC => {
                self.registers.set_bc(self.registers.bc().wrapping_sub(1));
            }
            U16Register::DE => {
                self.registers.set_de(self.registers.de().wrapping_sub(1));
            }
            U16Register::HL => {
                self.registers.set_hl(self.registers.hl().wrapping_sub(1));
            }
            U16Register::SP => {
                self.registers.sp = self.registers.sp.wrapping_sub(1);
            }
        }
    }

    fn add_SP(&mut self, val: i8) {
        let (new_value, overflowed) = self.registers.sp.overflowing_add(val as u16);
        self.registers.f.zero = false;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = (self.registers.sp & 0x0F00) + (val as u16 & 0x0F00) > 0x0F00;
        self.registers.f.carry = overflowed;
        self.registers.sp = new_value;
    }

    fn ld_HL(&mut self, val: i8) {
        let (new_value, overflowed) = self.registers.sp.overflowing_add(val as u16);
        self.registers.f.zero = false;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = (self.registers.sp & 0x0F00) + (val as u16 & 0x0F00) > 0x0F00;
        self.registers.f.carry = overflowed;
        self.registers.set_hl(new_value);
    }






































}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boop() {}
}
