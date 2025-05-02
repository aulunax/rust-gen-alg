use std::fmt;

/// Represents the registers in the DLX architecture.
#[derive(Debug, Clone)]
#[repr(i8)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
    R16 = 16,
    R17 = 17,
    R18 = 18,
    R19 = 19,
    R20 = 20,
    R21 = 21,
    R22 = 22,
    R23 = 23,
    R24 = 24,
    R25 = 25,
    R26 = 26,
    R27 = 27,
    R28 = 28,
    R29 = 29,
    R30 = 30,
    R31 = 31,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Register {
    /// Parses a string to a Register enum.
    /// The string should be in the format "R0", "R1", ..., "R31".
    /// Returns None if the string is not a valid register.
    pub fn parse_reg(input: &str) -> Option<Register> {
        if let Some(stripped) = input.strip_prefix('R') {
            if let Ok(num) = stripped.parse::<i8>() {
                match num {
                    0..=31 => match num {
                        0 => Some(Register::R0),
                        1 => Some(Register::R1),
                        2 => Some(Register::R2),
                        3 => Some(Register::R3),
                        4 => Some(Register::R4),
                        5 => Some(Register::R5),
                        6 => Some(Register::R6),
                        7 => Some(Register::R7),
                        8 => Some(Register::R8),
                        9 => Some(Register::R9),
                        10 => Some(Register::R10),
                        11 => Some(Register::R11),
                        12 => Some(Register::R12),
                        13 => Some(Register::R13),
                        14 => Some(Register::R14),
                        15 => Some(Register::R15),
                        16 => Some(Register::R16),
                        17 => Some(Register::R17),
                        18 => Some(Register::R18),
                        19 => Some(Register::R19),
                        20 => Some(Register::R20),
                        21 => Some(Register::R21),
                        22 => Some(Register::R22),
                        23 => Some(Register::R23),
                        24 => Some(Register::R24),
                        25 => Some(Register::R25),
                        26 => Some(Register::R26),
                        27 => Some(Register::R27),
                        28 => Some(Register::R28),
                        29 => Some(Register::R29),
                        30 => Some(Register::R30),
                        31 => Some(Register::R31),
                        _ => None,
                    },
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
