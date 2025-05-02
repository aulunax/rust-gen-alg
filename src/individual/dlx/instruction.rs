use std::fmt;
use std::path::Display;

use super::Opcode;
use super::Register;

#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: Opcode,
    registers: Vec<Register>,
    immidiate: i32,
}

impl Instruction {
    fn format_instr(&self) -> String {
        // String starting with opcode
        let mut output_str = self.opcode.to_string().clone();
        output_str.push(' ');

        // Iterator for format
        let mut format_chars = self.opcode.get_format().chars().peekable();

        // Format parser
        while let Some(c) = format_chars.next() {
            if c == 'r' {
                if let Some(digit_char) = format_chars.next() {
                    if let Some(idx) = digit_char.to_digit(10) {
                        if let Some(reg) = self.registers.get((idx - 1) as usize) {
                            output_str.push_str(&format!("{}", reg));
                        } else {
                            panic!("invalid register while printing instruction");
                        }
                    } else {
                        output_str.push(c);
                        output_str.push(digit_char);
                    }
                } else {
                    output_str.push(c);
                }
            } else if c == 'i' {
                output_str.push_str(&format!("0x{:04X}", self.immidiate));
            } else {
                output_str.push(c);
            }
        }

        output_str
    }

    fn parse_instr(instr: &str) -> Option<Self> {
        let input = instr.trim();
        let instr_parts: Vec<&str> = input.split_whitespace().collect();

        if instr_parts.len() == 0 {
            return None;
        }

        let opcode = Opcode::parse_instr(instr)?;

        Some(Instruction {
            opcode: opcode,
            registers: vec![],
            immidiate: 0,
        })
    }

    pub fn new(instr: &str) -> Self {
        todo!()
    }

    pub fn get_bytes(&self) -> u32 {
        todo!()
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            opcode: Opcode::NOP,
            registers: vec![],
            immidiate: 0,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_instr())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_display() {
        let inst = Instruction {
            opcode: Opcode::LDW,
            registers: vec![Register::R6, Register::R8],
            immidiate: 32,
        };

        let inst_add = Instruction {
            opcode: Opcode::ADD,
            registers: vec![Register::R4, Register::R3, Register::R2],
            immidiate: 32,
        };

        assert_eq!(inst.to_string(), "LDW R8, 0x0020(R6)");
        assert_eq!(inst_add.to_string(), "ADD R4, R3, R2");
    }
}
