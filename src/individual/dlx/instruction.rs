use super::Opcode;
use super::Register;

#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: Opcode,
    registers: Vec<Register>,
    immidiate: i32,
}

impl Instruction {
    fn parse_instr(instr: &str) -> Self {
        todo!()
    }

    pub fn new(instr: &str) -> Self {
        todo!()
    }

    pub fn get_bytes(&self) -> u32 {
        todo!()
    }

    pub fn to_string(&self) -> String {
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

// match &self.inst {
//             InstructionType::RType { opcode, registers } => {
//                 format!(
//                     "{} {}",
//                     opcode,
//                     registers
//                         .iter()
//                         .map(|r| r.to_string())
//                         .collect::<Vec<_>>()
//                         .join(", ")
//                 )
//             }
//             InstructionType::IType {
//                 opcode,
//                 registers,
//                 immediate,
//             } => {
//                 format!(
//                     "{} {}, {}",
//                     opcode,
//                     registers
//                         .iter()
//                         .map(|r| r.to_string())
//                         .collect::<Vec<_>>()
//                         .join(", "),
//                     immediate
//                 )
//             }
//             InstructionType::JType { opcode, address } => format!("{} 0x{:08X}", opcode, address),
//         }
