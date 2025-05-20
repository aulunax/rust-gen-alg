use std::path::Path;
use std::process::Command;

use rayon::iter::split;
use regex::Regex;

#[derive(Debug)]
pub struct EmulatorResult {
    success: bool,
    cycle_count: usize,
    unknown: bool,
    memory: Option<Vec<i32>>,
}

pub struct Emulator;

impl Emulator {
    pub fn run_python_emulator(code: &str) -> EmulatorResult {
        let output = Command::new("python3")
            .args(&["-B", "-u", "interface.py"])
            .args(&["--instr", code])
            .current_dir("src/emulator")
            .output()
            .expect("Failed to start python process");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
            Emulator::parse_emu_output(&stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Python raised an error:\n{}", stderr);
            EmulatorResult {
                success: false,
                cycle_count: 0,
                unknown: false,
                memory: None,
            }
        }
    }

    fn compress_spaces(s: &str) -> String {
        let re = Regex::new(r" +").unwrap();
        re.replace_all(s, " ").to_string()
    }

    fn parse_emu_output(output: &str) -> EmulatorResult {
        let split_output: Vec<&str> = output.splitn(4, '\n').collect();

        let success = split_output.get(0) == Some(&"Completed");

        let cycles: usize = split_output
            .get(1)
            .unwrap()
            .trim()
            .parse::<usize>()
            .expect("Invalid cycle count format. Not a valid number.");

        let unknown = split_output.get(2) == Some(&"True");

        if split_output.get(3) == None {
            return EmulatorResult {
                success: success,
                cycle_count: cycles,
                unknown: unknown,
                memory: None,
            };
        }

        let memory_string = Emulator::compress_spaces(split_output[3]);
        let memory_hex = memory_string.split_terminator(&[' ', '\n'][..]);

        let memory = memory_hex
            .map(|hex| i32::from_str_radix(hex, 16).unwrap())
            .collect();

        EmulatorResult {
            success: success,
            cycle_count: cycles,
            unknown: unknown,
            memory: Some(memory),
        }
    }
}

#[cfg(test)]
mod test {

    use rayon::prelude::*;

    use super::*;

    const SOI_CODE: &str = "ADDI R0, 0x00000010, R12\nADDI R0, 0x00000020, R11\nAND  R1, R0, R1\nMULI R12, 0x00000004, R12\nAND  R4, R0, R4\nMULI R11, 0x00000004, R11\nSUBI R12, 0x00000004, R13\nLDW  R7, 0x00000200(R1)\nADD  R4, R0, R5\nSUBI R13, 0x00000004, R17\nl1: AND  R2, R0, R2\nAND  R3, R0, R3\nSTW  R7, 0x00000280(R4)\nLDW  R9, 0x00000280(R5)\nLDW  R10, 0x000002C0(R2)\nl2: SUB  R5, R17, R14\nADD  R3, R9, R3\nADDI R2, 0x00000004, R2\nBRLE R14, h1\nSUB  R2, R12, R15\nADDI R5, 0x00000004, R5\nAND  R5, R0, R5\nh1: MUL  R3, R10, R3\nBRNZ R15, l2\nLDW  R9, 0x00000280(R5)\nLDW  R10, 0x000002C0(R2)\nSTW  R3, 0x00000300(R1)\nSUBI R4, 0x00000004, R4\nADDI R1, 0x00000004, R1\nNOP  \nBRGE R4, h2\nSUB  R1, R11, R15\nNOP  \nADD  R13, R0, R4\nh2: NOP  \nBRNZ R15, l1\nLDW  R7, 0x00000200(R1)\nADD  R4, R0, R5";
    const RUN_COUNT: usize = 10;
    const EXPECTED_CYCLES: usize = 6257;

    const MEMORY_OUTPUT_ADDR: usize = 192;
    const MEMORY_OUTPUT_SIZE: usize = 32;
    const MEMORY_OUTPUT_ADDR_END: usize = MEMORY_OUTPUT_ADDR + MEMORY_OUTPUT_SIZE;

    #[rustfmt::skip]
    const EXPECTED_MEMORY: [i32; 32] = [
        1, 3, 6, 10, 15, 21, 28, 36,
        45, 55, 66, 78, 91, 105, 120, 136,
        152, 168, 184, 200, 216, 232, 248, 264,
        280, 296, 312, 328, 344, 360, 376, 392,
    ];

    #[test]
    fn emu_test() {
        let result = Emulator::run_python_emulator(SOI_CODE);
        println!("{:?}", result);

        assert!(result.success);
        assert_eq!(result.cycle_count, EXPECTED_CYCLES);
        assert!(result.unknown);
        assert_eq!(
            &result.memory.as_ref().unwrap()[MEMORY_OUTPUT_ADDR..MEMORY_OUTPUT_ADDR_END],
            &EXPECTED_MEMORY
        );
    }

    #[test]
    fn emu_mt_test() {
        let codes: Vec<&str> = vec![SOI_CODE; RUN_COUNT];

        codes.par_iter().for_each(|code| {
            Emulator::run_python_emulator(code);
        });
    }
}
