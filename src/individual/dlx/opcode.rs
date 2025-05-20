use std::fmt;

use rand::seq::IndexedRandom;

use super::Register;

/// Represents the type of opcode in the instruction set architecture.
/// Currently unused
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpcodeType {
    RType,
    IType,
    JType,
}

/// Macro to define an `Opcode` enum and automatically generate methods and formatting
/// for each opcode variant. Each variant can have a unique `OpcodeType` and format string.
///
/// # Arguments
/// - `$name`: The name of the opcode variant.
/// - `$value`: The numeric value associated with the opcode (used in machine code).
/// - `$type`: The `OpcodeType` (RType, IType, JType) for the opcode.
/// - `$format`: A format string for how to represent the operands of the opcode.
///
macro_rules! define_opcodes {
    ($( $name:ident = $value:expr, $type:expr, $format:expr, $func:expr ),*) => {

        /// Enum representing the opcodes in the instruction set architecture.
        ///
        /// The byte value, type, and format string are associated with each variant.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u8)]
        pub enum Opcode {
            $(
                $name = $value,
            )*
        }

        impl Opcode {
            /// Returns the OpcodeType of the opcode.
            pub fn get_type(&self) -> OpcodeType {
                match self {
                    $(
                        Opcode::$name => $type,
                    )*
                }
            }

            /// Returns the format string of the opcode.
            pub fn get_format(&self) -> &str {
                match self {
                    $(
                        Opcode::$name => $format,
                    )*
                }
            }

            /// Execute an instruction based on the opcode
            pub fn execute(&self, regs: &mut Vec<&mut Register>, imm: &mut i32) -> () {
                match self {
                    $(
                        Opcode::$name => $func(regs, imm),
                    )*
                }
            }



            /// Parses an exact string to an Opcode enum.
            /// The string should be in the format "ADD", "LDW", etc.
            ///
            /// Returns None if the string is not a valid opcode.
            fn parse(input: &str) -> Option<Opcode> {
                match input {
                    $(
                        stringify!($name) => Some(Opcode::$name),
                    )*
                    _ => None,
                }
            }

            /// Parses an full instruction string to an Opcode enum.
            /// The string should be in the format "ADD R1 R2 R3", "LDW R1 R2", etc.
            ///
            /// Returns None if the string is not a valid instruction.
            pub fn parse_instr(input: &str) -> Option<Opcode> {
                let input = input.trim();
                let instr_parts: Vec<&str> = input.split_whitespace().collect();

                if instr_parts.len() == 0 {
                    return None;
                }

                if let Some(opcode) = Opcode::parse(instr_parts[0]) {
                    return Some(opcode);
                }
                None
            }
        }

        impl fmt::Display for Opcode {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = match self {
                    $(
                        Opcode::$name => stringify!($name),
                    )*
                };
                write!(f, "{}", s)
            }
        }

        pub const ALL_OPCODES: &[Opcode] = &[
            $(
                Opcode::$name,
            )*
        ];
    };
}

pub const ALL_RAND_OPCODES: &[Opcode] = &[
    Opcode::ADD,
    Opcode::SUB,
    Opcode::MUL,
    Opcode::DIV,
    Opcode::AND,
    Opcode::OR,
    Opcode::XOR,
    Opcode::SUBI,
    Opcode::MULI,
    Opcode::ADDI,
    Opcode::DIVI,
    Opcode::ANDI,
    Opcode::ORI,
    Opcode::XORI,
];

pub const BRANCH_OPCODES: &[Opcode] = &[
    Opcode::BRZ,
    Opcode::BRNZ,
    Opcode::BRGT,
    Opcode::BRGE,
    Opcode::BRLT,
    Opcode::BRLE,
];

impl Opcode {
    pub fn rand() -> Self {
        let mut rng = rand::rng();
        *ALL_RAND_OPCODES.choose(&mut rng).unwrap()
    }
}

fn nop_handler(regs: &mut Vec<&mut Register>, imm: &mut i32) {}

// Define opcodes here:
//  Opcode | Byte representation | Type | Format of instruction
define_opcodes! {
    NOP   = 0x00, OpcodeType::RType, "", nop_handler,
    ADD   = 0x01, OpcodeType::RType, "r1, r2, r3", nop_handler,
    LDW   = 0x02, OpcodeType::IType, "r2, i(r1)", nop_handler,
    STW   = 0x03, OpcodeType::IType, "r2, i(r1)", nop_handler,
    SUB   = 0x04, OpcodeType::RType, "r1, r2, r3", nop_handler,
    MUL   = 0x05, OpcodeType::RType, "r1, r2, r3", nop_handler,
    DIV   = 0x06, OpcodeType::RType, "r1, r2, r3", nop_handler,
    AND   = 0x07, OpcodeType::RType, "r1, r2, r3", nop_handler,
    OR    = 0x08, OpcodeType::RType, "r1, r2, r3", nop_handler,
    XOR   = 0x09, OpcodeType::RType, "r1, r2, r3", nop_handler,
    SUBI  = 0x0B, OpcodeType::IType, "r1, i, r2", nop_handler,
    MULI  = 0x0C, OpcodeType::IType, "r1, i, r2", nop_handler,
    ADDI  = 0x0A, OpcodeType::IType, "r1, i, r2", nop_handler,
    DIVI  = 0x0D, OpcodeType::IType, "r1, i, r2", nop_handler,
    ANDI  = 0x0E, OpcodeType::IType, "r1, i, r2", nop_handler,
    ORI   = 0x0F, OpcodeType::IType, "r1, i, r2", nop_handler,
    XORI  = 0x10, OpcodeType::IType, "r1, i, r2", nop_handler,
    BRZ   = 0x11, OpcodeType::IType, "r2, j", nop_handler,
    BRNZ  = 0x12, OpcodeType::IType, "r2, j", nop_handler,
    BRGT  = 0x13, OpcodeType::IType, "r2, j", nop_handler,
    BRGE  = 0x14, OpcodeType::IType, "r2, j", nop_handler,
    BRLT  = 0x15, OpcodeType::IType, "r2, j", nop_handler,
    BRLE  = 0x16, OpcodeType::IType, "r2, j", nop_handler
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_opcodes() {
        assert_eq!(Opcode::parse_instr("ADD R1 R2 R3"), Some(Opcode::ADD));
        assert_eq!(Opcode::parse_instr("LDW R1 R2"), Some(Opcode::LDW));
        assert_eq!(Opcode::parse_instr("NOP"), Some(Opcode::NOP));
        assert_eq!(Opcode::parse_instr("BRZ R4, 0xFFF8"), Some(Opcode::BRZ));
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(Opcode::parse_instr(""), None);
    }

    #[test]
    fn test_valid_format() {
        assert_eq!(Opcode::NOP.get_format(), "");
        assert_eq!(Opcode::LDW.get_format(), "r2, i(r1)");
        assert_eq!(Opcode::ADD.get_format(), "r1, r2, r3");
        assert_eq!(Opcode::BRZ.get_format(), "r2, j");
    }

    #[test]
    fn test_valid_type() {
        assert_eq!(Opcode::NOP.get_type(), OpcodeType::RType);
        assert_eq!(Opcode::LDW.get_type(), OpcodeType::IType);
        assert_eq!(Opcode::ADD.get_type(), OpcodeType::RType);
        assert_eq!(Opcode::BRZ.get_type(), OpcodeType::IType);
    }

    #[test]
    fn test_valid_display() {
        assert_eq!(Opcode::NOP.to_string(), "NOP");
        assert_eq!(Opcode::LDW.to_string(), "LDW");
        assert_eq!(Opcode::ADD.to_string(), "ADD");
        assert_eq!(Opcode::BRZ.to_string(), "BRZ");
    }
}
