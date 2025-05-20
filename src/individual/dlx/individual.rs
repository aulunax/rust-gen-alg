use std::cmp;
use std::fmt;

use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
    random,
};

use crate::individual::{dlx, genetic::Genetic};

use super::emu;
use super::opcode::BRANCH_OPCODES;
use super::{
    Opcode, Register,
    instruction::{MAX_IMMEDIATE_FOR_RAND, MAX_REGISTER_FOR_RAND},
};

const DLX_INDIV_MAX_SIZE: usize = 90;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Label {
    name: String,
    location: usize,
}

/// Individual for the DLX algorithm
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Individual {
    instructions: Vec<dlx::Instruction>,
    labels: Vec<Label>,
}

unsafe impl Sync for Individual {}
unsafe impl Send for Individual {}

impl Individual {
    fn last_nop_index(&self) -> usize {
        // let index = self
        //     .instructions
        //     .iter()
        //     .position(|a| a.get_opcode() == &Opcode::NOP);

        // match index {
        //     Some(n) => n,
        //     None => DLX_INDIV_MAX_SIZE,
        // }

        self.instructions
            .iter()
            .rposition(|a| a.get_opcode() != &Opcode::NOP)
            .unwrap_or(0)
    }

    fn duplicate_rand_instruction(&mut self) -> () {
        let last_pos = cmp::min(self.last_nop_index(), DLX_INDIV_MAX_SIZE - 1);

        if last_pos == 0 {
            return;
        }

        let mut rng = rand::rng();
        let position = rng.random_range(0..last_pos);

        let instr = self.instructions[position].clone();

        if BRANCH_OPCODES.contains(&instr.get_opcode()) {
            return;
        }

        // Make labels with location higher than position, higher by 1
        for label in self.labels.iter_mut() {
            if label.location > position {
                label.location += 1;
            }
        }

        self.instructions.insert(position, instr);
    }

    fn move_rand_instruction(&mut self) -> () {
        let last_pos = cmp::min(self.last_nop_index(), DLX_INDIV_MAX_SIZE - 1);

        if last_pos == 0 {
            return;
        }

        let mut rng = rand::rng();
        let position = rng.random_range(0..last_pos);
        let position2 = rng.random_range(0..last_pos);

        let instr = self.instructions[position].clone();

        if BRANCH_OPCODES.contains(&instr.get_opcode()) {
            return;
        }

        // Make labels with location higher than position, lower by 1
        for label in self.labels.iter_mut() {
            if label.location > position {
                label.location -= 1;
            }
        }

        // Make labels with location higher than position2, higher by 1
        for label in self.labels.iter_mut() {
            if label.location > position2 {
                label.location += 1;
            }
        }

        self.instructions.remove(position);
        self.instructions.insert(position2, instr);
    }

    fn remove_rand_instruction(&mut self) -> () {
        let last_pos = cmp::min(self.last_nop_index(), DLX_INDIV_MAX_SIZE - 1);

        if last_pos == 0 {
            return;
        }

        let mut rng = rand::rng();
        let mut position: usize = rng.random_range(0..last_pos);

        for i in 0..=5 {
            if i == 5 {
                return;
            }

            position = rng.random_range(0..last_pos);
            let instr = &self.instructions[position];

            if BRANCH_OPCODES.contains(&instr.get_opcode()) {
                continue;
            }

            if instr.get_opcode() == &Opcode::NOP {
                break;
            }
        }

        // Make labels with location higher than position, lower by 1
        for label in self.labels.iter_mut() {
            if label.location > position {
                label.location -= 1;
            }
        }

        self.instructions.remove(position);
        self.instructions.push(dlx::Instruction::default());
    }

    fn add_rand_instruction(&mut self) -> () {
        let last_pos = self.last_nop_index();

        if last_pos >= DLX_INDIV_MAX_SIZE {
            return;
        }

        let mut rng = rand::rng();

        let position = rng.random_range(0..=last_pos);

        let instr = dlx::Instruction::get_rand();

        // Make labels with location higher than position, higher by 1
        for label in self.labels.iter_mut() {
            if label.location > position {
                label.location += 1;
            }
        }

        self.instructions.insert(position, instr);
        self.instructions.pop();
    }

    fn change_rand_instruction(&mut self) -> () {
        let last_pos = cmp::min(self.last_nop_index(), DLX_INDIV_MAX_SIZE - 1);

        // Do nothing if only NOPs instructions
        if last_pos == 0 {
            return;
        }

        let mut rng = rand::rng();
        let position = rng.random_range(0..last_pos);

        let instr = dlx::Instruction::get_rand();

        self.instructions[position] = instr;
    }

    fn change_operands(&mut self) -> () {
        let last_pos = cmp::min(self.last_nop_index(), DLX_INDIV_MAX_SIZE - 1);

        // Do nothing if only NOPs instructions
        if last_pos == 0 {
            return;
        }

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

    // fn get_inner_loop_bounds(&self) -> (usize, usize) {
    //     (0, self.instructions.len() - 1)
    // }

    // fn get_setup_part_bounds(&self) -> (usize, usize) {
    //     (0, self.instructions.len() - 1)
    // }

    // fn get_outer_loop_top_part_bounds(&self) -> (usize, usize) {
    //     (0, self.instructions.len() - 1)
    // }

    // fn get_outer_loop_bottom_part_bounds(&self) -> (usize, usize) {
    //     (0, self.instructions.len() - 1)
    // }
}

#[rustfmt::skip]
const EXPECTED_MEMORY: [u32; 32] = [
    1, 3, 6, 10, 15, 21, 28, 36,
    45, 55, 66, 78, 91, 105, 120, 136,
    152, 168, 184, 200, 216, 232, 248, 264,
    280, 296, 312, 328, 344, 360, 376, 392,
];

const MEMORY_OUTPUT_ADDR: usize = 192;
const MEMORY_OUTPUT_SIZE: usize = 32;
const MEMORY_OUTPUT_ADDR_END: usize = MEMORY_OUTPUT_ADDR + MEMORY_OUTPUT_SIZE;

impl Genetic for Individual {
    fn fitness(&self) -> f32 {
        let result = emu::run_python_emulator(self.to_string());

        if !result.success {
            return 0.0;
        }

        if result.cycle_count > 20000 {
            return 0.0;
        }

        if &result.memory.as_ref().unwrap()[MEMORY_OUTPUT_ADDR..MEMORY_OUTPUT_ADDR_END]
            != &EXPECTED_MEMORY
        {
            return 1.0;
        }

        (20000 - result.cycle_count) as f32
    }

    fn generate() -> Self {
        Individual::new("NOP")
    }

    fn crossover(&self, other: &Self) -> Self {
        let mid_instr = self.last_nop_index() / 2;
        let mid_instr_other = other.last_nop_index() / 2;

        let first_half = self.instructions.split_at(mid_instr).0;
        let second_half = other.instructions.split_at(mid_instr_other).1;

        let mut child: Vec<_> = first_half
            .iter()
            .chain(second_half.iter())
            .cloned()
            .collect();

        if child.len() > DLX_INDIV_MAX_SIZE {
            child.truncate(DLX_INDIV_MAX_SIZE);
        }

        Individual {
            instructions: child,
            labels: self.labels.clone(),
        }
    }

    fn mutate(&mut self) -> () {
        let mut new_instr_chance = 0;
        let mut change_operands_chance = 0;
        let mut change_instruction_chance = 0;
        let mut remove_instr_chance = 50;
        let mut swap_instr_chance = 50;
        let mut duplicate_instr_chance = 10;

        if self.instructions.len() == 0 {
            new_instr_chance = 1000;
            change_operands_chance = 0;
            change_instruction_chance = 0;
            remove_instr_chance = 0;
            swap_instr_chance = 0;
            duplicate_instr_chance = 0;
        }

        let choices = [0, 1, 2, 3, 4, 5];
        let weights = [
            new_instr_chance,
            change_operands_chance,
            change_instruction_chance,
            remove_instr_chance,
            swap_instr_chance,
            duplicate_instr_chance,
        ];
        let dist = WeightedIndex::new(&weights).unwrap();

        let mut rng = rand::rng();
        match choices[dist.sample(&mut rng)] {
            0 => self.add_rand_instruction(),
            1 => self.change_operands(),
            2 => self.change_rand_instruction(),
            3 => self.remove_rand_instruction(),
            4 => self.move_rand_instruction(),
            5 => self.duplicate_rand_instruction(),
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
        let mut labels: Vec<Label> = vec![];

        let instrs_parts: Vec<&str> = input.split_terminator("\n").collect();

        println!("{:?}", instrs_parts);

        let mut instr_count = 0;
        for instr in instrs_parts {
            // get label, and remove it from string
            let labeless_instr = if instr.contains(':') {
                let split_instr = instr.split_once(':').unwrap();
                labels.push(Label {
                    name: split_instr.0.trim().to_string(),
                    location: instr_count,
                });
                split_instr.1
            } else {
                instr
            };

            instrs.push(dlx::Instruction::new(labeless_instr));
            instr_count += 1;
        }

        while instrs.len() < DLX_INDIV_MAX_SIZE {
            instrs.push(dlx::Instruction::default());
        }

        Individual {
            instructions: instrs,
            labels: labels,
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
        let mut counter = 0;
        for instr in &self.instructions {
            // if there is a label where, location == counter, write 'name:'
            if let Some(label) = self.labels.iter().find(|label| label.location == counter) {
                write!(f, "{}: ", label.name)?;
            }
            writeln!(f, "{}", instr)?;
            counter += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const RAW_INSTRUCTIONS: &str = r#"SUB R4, R4, R4
SUB R1, R1, R1
LDW R7, 0x00000200(R1)
STW R7, 0x00000280(R4)
SUB R3, R3, R3
ADD R4, R0, R5
SUB R2, R2, R2
LDW R8, 0x00000280(R5)
LDW R6, 0x000002C0(R2)
MUL R8, R6, R8
ADD R3, R8, R3
ADDI R5, 0x00000004, R5
ADDI R0, 0x0000003C, R8
SUB R5, R8, R8
BRLE R8, 0x00002134
SUB R5, R5, R5
ADDI R2, 0x00000004, R2
ADDI R0, 0x00000040, R8
SUB R2, R8, R8
BRLT R8, 0x00001234
STW R3, 0x00000300(R1)
SUBI R4, 0x00000004, R4
BRGE R4, 0x00003240
ADDI R0, 0x0000003C, R4
ADDI R1, 0x00000004, R1
ADDI R0, 0x00000080, R8
SUB R1, R8, R8
BRLT R8, 0x00000008"#;

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
        assert_eq!(indiv.last_nop_index(), RAW_INSTRUCTIONS_LEN);

        let indiv2 = Individual::generate();
        assert_eq!(indiv2.last_nop_index(), 0);
    }

    #[test]
    fn test_dlx_rand_instruction() {
        let mut indiv = Individual::generate();
        for _ in 0..10 {
            indiv.add_rand_instruction();
        }

        print!("{}", indiv);

        assert_eq!(indiv.last_nop_index(), 10);
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

        assert_eq!(indiv_changed.last_nop_index(), RAW_INSTRUCTIONS_LEN);
        assert_eq!(indiv_changed.instructions.len(), DLX_INDIV_MAX_SIZE);
    }

    #[test]
    fn test_dlx_change_operands_when_none() {
        let indiv = Individual::default();
        let mut indiv_changed = Individual::default();

        for _ in 0..(DLX_INDIV_MAX_SIZE + 10) {
            indiv_changed.change_operands();
        }

        print!("{}\n", indiv);
        print!("{}\n", indiv_changed);

        assert_eq!(indiv_changed.last_nop_index(), 0);
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

        assert_eq!(indiv_changed.last_nop_index(), RAW_INSTRUCTIONS_LEN);
        assert_eq!(indiv_changed.instructions.len(), DLX_INDIV_MAX_SIZE);
    }

    #[test]
    fn test_dlx_change_instructions_when_none() {
        let indiv = Individual::default();
        let mut indiv_changed = Individual::default();

        for _ in 0..(DLX_INDIV_MAX_SIZE + 10) {
            indiv_changed.change_rand_instruction();
        }

        print!("{}\n", indiv);
        print!("{}\n", indiv_changed);

        assert_eq!(indiv_changed.last_nop_index(), 0);
        assert_eq!(indiv_changed.instructions.len(), DLX_INDIV_MAX_SIZE);
    }

    #[test]
    fn test_dlx_rand_instruction_over_limit() {
        let mut indiv = Individual::generate();
        for _ in 0..(DLX_INDIV_MAX_SIZE + 10) {
            indiv.add_rand_instruction();
        }

        print!("{}", indiv);

        assert_eq!(indiv.last_nop_index(), DLX_INDIV_MAX_SIZE);
        assert_eq!(indiv.instructions.len(), DLX_INDIV_MAX_SIZE);
    }

    const SOI_ALG_START: &str = "ADDI R0, 0x00000010, R12\nADDI R0, 0x00000020, R11\nAND R1, R0, R1\nMULI R12, 0x00000004, R12\nAND R4, R0, R4\nMULI R11, 0x00000004, R11\nSUBI R12, 0x00000004, R13\nLDW R7, 0x00000200(R1)\nADD R4, R0, R5\nSUBI R13, 0x00000004, R17\nl1: AND R2, R0, R2\nAND R3, R0, R3\nSTW R7, 0x00000280(R4)\nLDW R9, 0x00000280(R5)\nLDW R10, 0x000002C0(R2)\nl2: SUB R5, R17, R14\nADD R3, R9, R3\nADDI R2, 0x00000004, R2\nBRLE R14, h1\nSUB R2, R12, R15\nADDI R5, 0x00000004, R5\nAND R5, R0, R5\nh1: MUL R3, R10, R3\nBRNZ R15, l2\nLDW R9, 0x00000280(R5)\nLDW R10, 0x000002C0(R2)\nSTW R3, 0x00000300(R1)\nSUBI R4, 0x00000004, R4\nADDI R1, 0x00000004, R1\nNOP\nBRGE R4, h2\nSUB R1, R11, R15\nNOP\nADD R13, R0, R4\nh2: NOP\nBRNZ R15, l1\nLDW R7, 0x00000200(R1)\nADD R4, R0, R5";

    #[test]
    fn test_dlx_instruction_labels() {
        let indiv = Individual::new(SOI_ALG_START);

        assert_eq!(indiv.to_string(), format!("{}\nNOP\nNOP\n", SOI_ALG_START));
    }
}
