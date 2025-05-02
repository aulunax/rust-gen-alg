use std::fmt;
use std::path::Display;

use regex::Regex;

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

        // Iterator for format
        let mut format_chars = self.opcode.get_format().chars().peekable();

        if self.opcode.get_format() != "" {
            output_str.push(' ');
        }

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

    fn split_operands(input: &str) -> Vec<String> {
        let re = Regex::new(r"0x[0-9a-fA-F]+|R\d+|\d+").unwrap();
        re.find_iter(input)
            .map(|m| m.as_str().to_string())
            .collect()
    }

    fn parse_instr(instr: &str) -> Option<Self> {
        let input = instr.trim();
        let instr_parts: Vec<&str> = input.split_whitespace().collect();

        if instr_parts.len() == 0 {
            return None;
        }

        let opcode = Opcode::parse_instr(instr)?;

        let mut format_chars = opcode.get_format().chars().peekable();
        let operands_str = if let Some((_, rest)) = input.split_once(' ') {
            rest
        } else {
            ""
        };

        let operands_parts: Vec<String> = Instruction::split_operands(operands_str);

        let mut regs: Vec<(usize, Register)> = vec![];
        let mut immidiate: i32 = 0;

        let mut current_operand_index = 0;

        while let Some(c) = format_chars.next() {
            if c == 'r' {
                if let Some(digit_char) = format_chars.next() {
                    if let Some(idx) = digit_char.to_digit(10) {
                        regs.push((
                            (idx - 1) as usize,
                            Register::parse_reg(&operands_parts[current_operand_index]).unwrap(),
                        ));
                        current_operand_index += 1;
                    }
                }
            } else if c == 'i' {
                let cleaned_immidiate =
                    operands_parts[current_operand_index].trim_start_matches("0x");
                immidiate = i32::from_str_radix(cleaned_immidiate, 16).unwrap();
                current_operand_index += 1;
            }
        }

        let max_index = regs.iter().map(|(index, _)| *index).max().unwrap_or(0);
        let mut registers = vec![Register::R0; max_index + 1];
        for (index, reg) in regs {
            registers[index] = reg;
        }

        Some(Instruction {
            opcode: opcode,
            registers: registers,
            immidiate: immidiate,
        })
    }

    pub fn new(instr: &str) -> Self {
        Instruction::parse_instr(instr).unwrap()
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

        let inst_nop = Instruction {
            opcode: Opcode::NOP,
            registers: vec![],
            immidiate: 0,
        };

        assert_eq!(inst.to_string(), "LDW R8, 0x0020(R6)");
        assert_eq!(inst_add.to_string(), "ADD R4, R3, R2");
        assert_eq!(inst_nop.to_string(), "NOP");
    }

    #[test]
    fn test_instruction_parse() {
        let add_inst = Instruction::new("ADD R4, R3, R2");
        let ldw_inst = Instruction::new("LDW R8, 0x0020(R6)");
        let nop_inst = Instruction::new("NOP");

        assert_eq!(ldw_inst.to_string(), "LDW R8, 0x0020(R6)");
        assert_eq!(add_inst.to_string(), "ADD R4, R3, R2");
        assert_eq!(nop_inst.to_string(), "NOP");
    }
}
