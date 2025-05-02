use std::fmt;

use crate::individual::{dlx, genetic::Genetic};

use super::Instruction;

const DLX_INDIV_MAX_SIZE: usize = 40;

/// Individual for the DLX algorithm
#[derive(Clone, Debug)]
pub struct Individual {
    instructions: Vec<dlx::Instruction>,
}

impl Genetic for Individual {
    fn fitness(&self) -> f32 {
        todo!()
    }

    fn generate() -> Self {
        todo!()
    }

    fn crossover(&self, other: &Self) -> Self {
        todo!()
    }

    fn mutate(&mut self) -> () {
        todo!()
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
        let mut instrs: Vec<Instruction> = vec![];

        let instrs_parts: Vec<&str> = input.split_terminator("\n").collect();

        for instr in instrs_parts {
            instrs.push(Instruction::new(instr));
        }

        while instrs.len() < DLX_INDIV_MAX_SIZE {
            instrs.push(Instruction::default());
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
}
