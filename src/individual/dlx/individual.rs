use std::fmt;

use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
    random, random_range, rng,
};

use crate::individual::{dlx, genetic::Genetic};

use super::{
    Opcode, Register,
    instruction::{self, MAX_IMMEDIATE_FOR_RAND, MAX_REGISTER_FOR_RAND},
};

const DLX_INDIV_MAX_SIZE: usize = 40;

/// Individual for the DLX algorithm
#[derive(Clone, Debug)]
pub struct Individual {
    instructions: Vec<dlx::Instruction>,
}

impl Individual {
    fn first_nop_index(&self) -> usize {
        let index = self
            .instructions
            .iter()
            .position(|a| a.get_opcode() == &Opcode::NOP);

        match index {
            Some(n) => n,
            None => DLX_INDIV_MAX_SIZE,
        }
    }

    fn add_rand_instruction(&mut self) -> () {
        let last_pos = self.first_nop_index();

        if last_pos >= DLX_INDIV_MAX_SIZE {
            return;
        }

        let mut rng = rand::rng();

        let position = rng.random_range(0..=last_pos);

        let instr = dlx::Instruction::get_rand();

        self.instructions.insert(position, instr);
        self.instructions.pop();
    }

    fn change_rand_instruction(&mut self) -> () {
        let last_pos = self.first_nop_index();

        if last_pos >= DLX_INDIV_MAX_SIZE {
            return;
        }

        let mut rng = rand::rng();

        let position = rng.random_range(0..last_pos);

        let instr = dlx::Instruction::get_rand();

        self.instructions[position] = instr;
    }

    fn change_operands(&mut self) -> () {
        let last_pos = self.first_nop_index();
        let mut rng = rand::rng();
        let rand_index = rng.random_range(0..last_pos);

        let instr_type = self.instructions[rand_index].get_opcode().get_type();

        match instr_type {
            dlx::opcode::OpcodeType::RType => self.rand_change_reg_in_instruction(rand_index),
            dlx::opcode::OpcodeType::IType => {
                if random::<bool>() {
                    self.rand_change_reg_in_instruction(rand_index);
                } else {
                    self.rand_change_imm_in_instruction(rand_index);
                }
            }
            dlx::opcode::OpcodeType::JType => todo!(),
        }
    }

    fn rand_change_reg_in_instruction(&mut self, index: usize) -> () {
        let register = Register::rand_up_to(MAX_REGISTER_FOR_RAND).unwrap();

        self.instructions[index].set_register(None, register);
    }

    fn rand_change_imm_in_instruction(&mut self, index: usize) -> () {
        let mut rng = rand::rng();
        let imm = rng.random_range(-MAX_IMMEDIATE_FOR_RAND..MAX_IMMEDIATE_FOR_RAND);

        self.instructions[index].set_immidiate(imm);
    }

    fn get_inner_loop_bounds(&self) -> (usize, usize) {
        (0, self.instructions.len() - 1)
    }

    fn get_setup_part_bounds(&self) -> (usize, usize) {
        (0, self.instructions.len() - 1)
    }

    fn get_outer_loop_top_part_bounds(&self) -> (usize, usize) {
        (0, self.instructions.len() - 1)
    }

    fn get_outer_loop_bottom_part_bounds(&self) -> (usize, usize) {
        (0, self.instructions.len() - 1)
    }
}

impl Genetic for Individual {
    fn fitness(&self) -> f32 {
        1.0
    }

    fn generate() -> Self {
        Individual::default()
    }

    fn crossover(&self, other: &Self) -> Self {
        let mid_instr = self.first_nop_index() / 2;
        let mid_instr_other = other.first_nop_index() / 2;

        let first_half = self.instructions.split_at(mid_instr).0;
        let second_half = other.instructions.split_at(mid_instr_other).1;

        let child: Vec<_> = first_half
            .iter()
            .chain(second_half.iter())
            .cloned()
            .collect();

        Individual {
            instructions: child,
        }
    }

    fn mutate(&mut self) -> () {
        let choices = [0, 1, 2];
        let weights = [5, 50, 10];
        let dist = WeightedIndex::new(&weights).unwrap();

        let mut rng = rand::rng();
        match choices[dist.sample(&mut rng)] {
            0 => self.add_rand_instruction(),
            1 => self.change_operands(),
            2 => self.change_rand_instruction(),
            _ => unreachable!(),
        }
    }
}

impl Individual {
    /// Parses a string of instructions into an Individual.
    ///
    /// Instructions are separated by newlines.
    /// ## Arguments
    /// * `input` - A string containing the instructions to parse.
    /// ## Returns
    /// * An `Individual` containing the parsed instructions.
    pub fn parse(input: &str) -> Self {
        let mut instrs: Vec<dlx::Instruction> = vec![];

        let instrs_parts: Vec<&str> = input.split_terminator("\n").collect();

        for instr in instrs_parts {
            instrs.push(dlx::Instruction::new(instr));
        }

        while instrs.len() < DLX_INDIV_MAX_SIZE {
            instrs.push(dlx::Instruction::default());
        }

        Individual {
            instructions: instrs,
        }
    }

    /// Creates a new Individual from a string of instructions.
    ///
    /// Instructions are separated by newlines.
    /// ## Arguments
    /// * `input` - A string containing the instructions to parse.
    /// ## Returns
    /// * An `Individual` containing the parsed instructions.
    pub fn new(input: &str) -> Self {
        Individual::parse(input)
    }
}

impl Default for Individual {
    fn default() -> Self {
        Individual::new("NOP")
    }
}

impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instr in &self.instructions {
            writeln!(f, "{}", instr)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const RAW_INSTRUCTIONS: &str = r#"SUB R4, R4, R4
SUB R1, R1, R1
LDW R7, 0x0200(R1)
STW R7, 0x0280(R4)
SUB R3, R3, R3
ADD R4, R0, R5
SUB R2, R2, R2
LDW R8, 0x0280(R5)
LDW R6, 0x02C0(R2)
MUL R8, R6, R8
ADD R3, R8, R3
ADDI R5, 0x0004, R5
ADDI R0, 0x003C, R8
SUB R5, R8, R8
BRLE R8, 0x2134
SUB R5, R5, R5
ADDI R2, 0x0004, R2
ADDI R0, 0x0040, R8
SUB R2, R8, R8
BRLT R8, 0x1234
STW R3, 0x0300(R1)
SUBI R4, 0x0004, R4
BRGE R4, 0x3240
ADDI R0, 0x003C, R4
ADDI R1, 0x0004, R1
ADDI R0, 0x0080, R8
SUB R1, R8, R8
BRLT R8, 0x0008"#;

    const RAW_INSTRUCTIONS_LEN: usize = 28;

    #[test]
    fn test_dlx_indiv_parse() {
        let indiv = Individual::parse(RAW_INSTRUCTIONS);

        print!("{}", indiv);

        assert!(
            indiv.to_string().contains(RAW_INSTRUCTIONS),
            "Individual doesn't contain exact instructions it was given."
        );
        assert!(
            indiv.instructions.len() == DLX_INDIV_MAX_SIZE,
            "Individual is not filled with correct amount of instructions."
        );
    }

    #[test]
    fn test_dlx_indiv_generate() {
        let indiv = Individual::generate();
        assert!(indiv.to_string().contains("NOP"));
        assert!(indiv.instructions.len() == DLX_INDIV_MAX_SIZE);
    }

    #[test]
    fn test_dlx_last_index() {
        let indiv = Individual::parse(RAW_INSTRUCTIONS);
        assert_eq!(indiv.first_nop_index(), RAW_INSTRUCTIONS_LEN);

        let indiv2 = Individual::generate();
        assert_eq!(indiv2.first_nop_index(), 0);
    }

    #[test]
    fn test_dlx_rand_instruction() {
        let mut indiv = Individual::generate();
        for _ in 0..10 {
            indiv.add_rand_instruction();
        }

        print!("{}", indiv);

        assert_eq!(indiv.first_nop_index(), 10);
        assert_eq!(indiv.instructions.len(), DLX_INDIV_MAX_SIZE);
    }

    #[test]
    fn test_dlx_change_operands() {
        let indiv = Individual::parse(RAW_INSTRUCTIONS);
        let mut indiv_changed = Individual::parse(RAW_INSTRUCTIONS);

        for _ in 0..(DLX_INDIV_MAX_SIZE + 10) {
            indiv_changed.change_operands();
        }

        print!("{}\n", indiv);
        print!("{}\n", indiv_changed);

        assert_eq!(indiv_changed.first_nop_index(), RAW_INSTRUCTIONS_LEN);
        assert_eq!(indiv_changed.instructions.len(), DLX_INDIV_MAX_SIZE);
    }

    #[test]
    fn test_dlx_change_instructions() {
        let indiv = Individual::parse(RAW_INSTRUCTIONS);
        let mut indiv_changed = Individual::parse(RAW_INSTRUCTIONS);

        for _ in 0..(DLX_INDIV_MAX_SIZE + 10) {
            indiv_changed.change_rand_instruction();
        }

        print!("{}\n", indiv);
        print!("{}\n", indiv_changed);

        assert_eq!(indiv_changed.first_nop_index(), RAW_INSTRUCTIONS_LEN);
        assert_eq!(indiv_changed.instructions.len(), DLX_INDIV_MAX_SIZE);
    }

    #[test]
    fn test_dlx_rand_instruction_over_limit() {
        let mut indiv = Individual::generate();
        for _ in 0..(DLX_INDIV_MAX_SIZE + 10) {
            indiv.add_rand_instruction();
        }

        print!("{}", indiv);

        assert_eq!(indiv.first_nop_index(), DLX_INDIV_MAX_SIZE);
        assert_eq!(indiv.instructions.len(), DLX_INDIV_MAX_SIZE);
    }
}
