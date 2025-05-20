use std::time::Instant;

use gen_alg::{genalg::GenAlg, individual::dlx::Individual, *};
use rayon::vec;
const SOI_ALG_START: &str = "ADDI R0, 0x00000010, R12\nADDI R0, 0x00000020, R11\nAND R1, R0, R1\nMULI R12, 0x00000004, R12\nAND R4, R0, R4\nMULI R11, 0x00000004, R11\nSUBI R12, 0x00000004, R13\nLDW R7, 0x00000200(R1)\nADD R4, R0, R5\nSUBI R13, 0x00000004, R17\nl1: AND R2, R0, R2\nAND R3, R0, R3\nSTW R7, 0x00000280(R4)\nLDW R9, 0x00000280(R5)\nLDW R10, 0x000002C0(R2)\nl2: SUB R5, R17, R14\nADD R3, R9, R3\nADDI R2, 0x00000004, R2\nBRLE R14, h1\nSUB R2, R12, R15\nADDI R5, 0x00000004, R5\nAND R5, R0, R5\nh1: MUL R3, R10, R3\nBRNZ R15, l2\nLDW R9, 0x00000280(R5)\nLDW R10, 0x000002C0(R2)\nSTW R3, 0x00000300(R1)\nSUBI R4, 0x00000004, R4\nADDI R1, 0x00000004, R1\nNOP\nBRGE R4, h2\nSUB R1, R11, R15\nNOP\nADD R13, R0, R4\nh2: NOP\nBRNZ R15, l1\nLDW R7, 0x00000200(R1)\nADD R4, R0, R5";

#[test]
fn test_dlx_gen_alg() {
    let start_timer = Instant::now();

    let mut gen_alg =
        GenAlg::<Individual>::new(100, Some(&vec![Individual::new(SOI_ALG_START); 10]));

    let best = gen_alg.run_genetic_algorithm(10, 0.5, 0.3, 5).unwrap();

    println!("{}\n Fitness: {}", best.obj(), best.fitness());
    //print!("{:?}\n", gen_alg.population_history());

    let duration = start_timer.elapsed();
    println!("Time elapsed: {:?}", duration);
}
