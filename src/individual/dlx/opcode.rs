use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    ($( $name:ident = $value:expr, $type:expr, $format:expr ),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[repr(u8)]
        pub enum Opcode {
            $(
                $name = $value,
            )*
        }

        impl Opcode {
            pub fn get_type(&self) -> OpcodeType {
                match self {
                    $(
                        Opcode::$name => $type,
                    )*
                }
            }

            pub fn get_format(&self) -> &str {
                match self {
                    $(
                        Opcode::$name => $format,
                    )*
                }
            }

            fn parse(input: &str) -> Option<Opcode> {
                match input {
                    $(
                        stringify!($name) => Some(Opcode::$name),
                    )*
                    _ => None,
                }
            }

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
    };
}

// Define opcodes here:
define_opcodes! {
    NOP = 0x00, OpcodeType::RType, "",
    ADD = 0x01, OpcodeType::RType, "r1, r2, r3",
    LDW = 0x02, OpcodeType::IType, "r2, i(r1)"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_opcodes() {
        assert_eq!(Opcode::parse_instr("ADD R1 R2 R3"), Some(Opcode::ADD));
        assert_eq!(Opcode::parse_instr("LDW R1 R2"), Some(Opcode::LDW));
        assert_eq!(Opcode::parse_instr("NOP"), Some(Opcode::NOP));
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
    }

    #[test]
    fn test_valid_type() {
        assert_eq!(Opcode::NOP.get_type(), OpcodeType::RType);
        assert_eq!(Opcode::LDW.get_type(), OpcodeType::IType);
        assert_eq!(Opcode::ADD.get_type(), OpcodeType::RType);
    }

    #[test]
    fn test_valid_display() {
        assert_eq!(Opcode::NOP.to_string(), "NOP");
        assert_eq!(Opcode::LDW.to_string(), "LDW");
        assert_eq!(Opcode::ADD.to_string(), "ADD");
    }
}
