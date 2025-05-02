use std::fmt;

#[derive(Debug, Clone)]
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
/// # Example:
/// ```rust
/// define_opcodes! {
///     NOP = 0x00, OpcodeType::RType, "",
///     ADD = 0x01, OpcodeType::RType, "r1, r2, r3",
///     LDW = 0x02, OpcodeType::IType, "r2, i(r1)"
/// }
/// ```
macro_rules! define_opcodes {
    ($( $name:ident = $value:expr, $type:expr, $format:expr ),*) => {
        #[derive(Debug, Clone)]
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
