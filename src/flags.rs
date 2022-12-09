const ZERO_FLAG_BYTE_POSITION: usize = 7;
const SUBTRACTION_FLAG_BYTE_POSITION: usize = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: usize = 5;
const CARRY_FLAG_BYTE_POSITION: usize = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Flags {
    pub zero: bool,
    pub subtraction: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl std::convert::From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Self {
            zero: (byte & (1 << ZERO_FLAG_BYTE_POSITION)) != 0,
            subtraction: (byte & (1 << SUBTRACTION_FLAG_BYTE_POSITION)) != 0,
            half_carry: (byte & (1 << HALF_CARRY_FLAG_BYTE_POSITION)) != 0,
            carry: (byte & (1 << CARRY_FLAG_BYTE_POSITION)) != 0,
        }
    }
}

impl std::convert::From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        (flags.zero as u8) << ZERO_FLAG_BYTE_POSITION
            | (flags.subtraction as u8) << SUBTRACTION_FLAG_BYTE_POSITION
            | (flags.half_carry as u8) << HALF_CARRY_FLAG_BYTE_POSITION
            | (flags.carry as u8) << CARRY_FLAG_BYTE_POSITION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_from_u8() {
        let flags = Flags {
            zero: true,
            subtraction: true,
            half_carry: true,
            carry: true,
        };
        assert_eq!(flags, Flags::from(0xF0));

        let flags = Flags {
            zero: true,
            ..Flags::default()
        };
        assert_eq!(flags, Flags::from(0x80));

        let flags = Flags {
            subtraction: true,
            ..Flags::default()
        };
        assert_eq!(flags, Flags::from(0x40));

        let flags = Flags {
            half_carry: true,
            ..Flags::default()
        };
        assert_eq!(flags, Flags::from(0x20));

        let flags = Flags {
            carry: true,
            ..Flags::default()
        };
        assert_eq!(flags, Flags::from(0x10));

        let flags = Flags::default();
        assert_eq!(flags, Flags::from(0x00));
    }

    #[test]
    fn u8_from_flags() {
        let flags = Flags {
            zero: true,
            subtraction: true,
            half_carry: true,
            carry: true,
        };
        assert_eq!(u8::from(flags), 0xF0);

        let flags = Flags {
            zero: true,
            ..Flags::default()
        };
        assert_eq!(u8::from(flags), 0x80);

        let flags = Flags {
            subtraction: true,
            ..Flags::default()
        };
        assert_eq!(u8::from(flags), 0x40);

        let flags = Flags {
            half_carry: true,
            ..Flags::default()
        };
        assert_eq!(u8::from(flags), 0x20);

        let flags = Flags {
            carry: true,
            ..Flags::default()
        };
        assert_eq!(u8::from(flags), 0x10);

        let flags = Flags::default();
        assert_eq!(u8::from(flags), 0x00);
    }
}
