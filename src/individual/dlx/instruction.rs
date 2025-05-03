use core::panic;
use rand::Rng;
use rand::rngs::ThreadRng;
use regex::Regex;
use std::fmt;

use super::Opcode;
use super::Register;

pub const MAX_REGISTER_FOR_RAND: usize = 10;
pub const MAX_IMMEDIATE_FOR_RAND: i32 = 200;

#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: Opcode,
    registers: Vec<Register>,
    immidiate: i32,
}

impl Instruction {
    pub fn get_opcode(&self) -> &Opcode {
        &self.opcode
    }

    pub fn get_immidiate(&self) -> i32 {
        self.immidiate
    }

    pub fn get_rand() -> Self {
        let r_opcode = Opcode::rand();

        let r_regs = vec![
            Register::rand_up_to(MAX_REGISTER_FOR_RAND).unwrap(),
            Register::rand_up_to(MAX_REGISTER_FOR_RAND).unwrap(),
            Register::rand_up_to(MAX_REGISTER_FOR_RAND).unwrap(),
        ];

        let mut rng = rand::rng();

        let r_imm = rng.random_range(-MAX_IMMEDIATE_FOR_RAND..MAX_IMMEDIATE_FOR_RAND) * 4;

        Instruction {
            opcode: r_opcode,
            registers: r_regs,
            immidiate: r_imm,
        }
    }

    /// Sets a register to a selected or random register
    ///
    /// ## Arguments
    /// * `pos` - The position of the register to set. If None, a random register will be set.
    /// * `reg` - The register to set.
    pub fn set_register(&mut self, pos: Option<usize>, reg: Register) {
        match pos {
            Some(p) => self.registers[p] = reg,
            None => {
                let mut rng = rand::rng();
                let reg_index = rng.random_range(0..self.registers.len());
                self.registers[reg_index] = reg;
            }
        }
    }

    pub fn set_immidiate(&mut self, imm: i32) {
        self.immidiate = imm;
    }

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
            } else if c == 'i' || c == 'j' {
                output_str.push_str(&format!("0x{:04X}", self.immidiate));
            } else {
                output_str.push(c);
            }
        }

        output_str
    }

    /// Splits the operands of an instruction into a vector of strings.
    ///
    /// Assumes that operands consist of only alphanumeric characters, so any not alphanumeric characters are treated as separators.
    fn split_operands(input: &str) -> Vec<String> {
        let re = Regex::new(r"0x[0-9a-fA-F]+|R\d+|\d+").unwrap();
        re.find_iter(input)
            .map(|m| m.as_str().to_string())
            .collect()
    }

    /// Parses an instruction string into an Instruction struct.
    ///
    /// The instruction string should be in the format "OPCODE FORMAT", where OPCODE is the opcode and FORMAT is the format specified by the opcode.
    /// ## Arguments
    /// * `instr` - The instruction string to parse.
    ///
    /// ## Returns
    /// * `Some(Instruction)` if the instruction string is valid and can be parsed.
    /// * `None` if the instruction string is invalid or cannot be parsed.
    fn parse_instr(instr: &str) -> Option<Self> {
        // First get the opcode
        let input = instr.trim();
        let instr_parts: Vec<&str> = input.split_whitespace().collect();
        if instr_parts.len() == 0 {
            return None;
        }

        let opcode = Opcode::parse_instr(instr)?;

        // Get format from opcode
        let mut format_chars = opcode.get_format().chars().peekable();

        // Get operands vector from instruction
        let operands_str = if let Some((_, rest)) = input.split_once(' ') {
            rest
        } else {
            ""
        };
        let operands_parts: Vec<String> = Instruction::split_operands(operands_str);

        // Temp output vars
        let mut regs: Vec<(usize, Register)> = vec![];
        let mut immidiate: i32 = 0;

        let mut current_operand_index = 0;

        // Format parser
        while let Some(c) = format_chars.next() {
            if c == 'r' {
                if let Some(digit_char) = format_chars.next() {
                    if let Some(idx) = digit_char.to_digit(10) {
                        if current_operand_index < operands_parts.len() {
                            regs.push((
                                (idx - 1) as usize,
                                Register::parse_reg(&operands_parts[current_operand_index])
                                    .unwrap(),
                            ));
                        }
                        current_operand_index += 1;
                    }
                }
            } else if c == 'i' || c == 'j' {
                if current_operand_index < operands_parts.len() {
                    let cleaned_immidiate =
                        operands_parts[current_operand_index].trim_start_matches("0x");
                    immidiate = i32::from_str_radix(cleaned_immidiate, 16).unwrap();
                }
                current_operand_index += 1;
            }
        }

        // Check if correct number of operands was used
        if current_operand_index != operands_parts.len() {
            panic!(
                "Invalid number of operands expected {} operands, got {}",
                current_operand_index,
                operands_parts.len()
            );
        }

        // Convert from (index, Register) to Register vector
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
    /// Returns a default instruction with opcode NOP and no operands.
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
    fn test_default_instruction() {
        let inst = Instruction::default();
        assert_eq!(inst.opcode, Opcode::NOP);
        assert_eq!(inst.registers.len(), 0);
        assert_eq!(inst.immidiate, 0);
    }

    #[test]
    #[should_panic(expected = "Invalid number of operands expected 3 operands, got 4")]
    fn test_parse_too_many_operands() {
        let inst = Instruction::parse_instr("ADD R1, R2, R3, R4");
        assert!(inst.is_none());
    }

    #[test]
    #[should_panic(expected = "Invalid number of operands expected 0 operands, got 4")]
    fn test_parse_too_many_operands2() {
        let inst = Instruction::parse_instr("NOP R1, R2, R3, R4");
        assert!(inst.is_none());
    }

    #[test]
    #[should_panic(expected = "Invalid number of operands expected 3 operands, got 2")]
    fn test_parse_too_less_operands() {
        let inst = Instruction::parse_instr("ADD R1, R2");
        assert!(inst.is_none());
    }

    #[test]
    #[should_panic(expected = "Invalid number of operands expected 3 operands, got 0")]
    fn test_parse_too_less_operands2() {
        let inst = Instruction::parse_instr("ADD");
        assert!(inst.is_none());
    }

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

        let inst_brz = Instruction {
            opcode: Opcode::BRZ,
            registers: vec![Register::R4, Register::R3],
            immidiate: 32,
        };

        assert_eq!(inst.to_string(), "LDW R8, 0x0020(R6)");
        assert_eq!(inst_add.to_string(), "ADD R4, R3, R2");
        assert_eq!(inst_nop.to_string(), "NOP");
        assert_eq!(inst_brz.to_string(), "BRZ R3, 0x0020");
    }

    #[test]
    fn test_instruction_parse() {
        let add_inst = Instruction::new("ADD R4, R3, R2");
        let ldw_inst = Instruction::new("LDW R8, 0x0020(R6)");
        let nop_inst = Instruction::new("NOP");
        let brz_inst = Instruction::new("BRZ R3, 0x0020");

        assert_eq!(ldw_inst.to_string(), "LDW R8, 0x0020(R6)");
        assert_eq!(add_inst.to_string(), "ADD R4, R3, R2");
        assert_eq!(nop_inst.to_string(), "NOP");
        assert_eq!(brz_inst.to_string(), "BRZ R3, 0x0020");
    }
}
