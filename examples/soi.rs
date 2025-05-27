use gen_alg::{genalg::GenAlg, individual::dlx::Individual};
use std::{env, fs, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <code_file_path> [pop_size] [gen_num]", args[0]);
        std::process::exit(1);
    }

    let code_file_path = &args[1];
    let code_string = fs::read_to_string(code_file_path).expect("Failed to read code file");

    let pop_size = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(20);

    let gen_num = args
        .get(3)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(50);

    let start_timer = Instant::now();

    let mut gen_alg = GenAlg::<Individual>::new(
        pop_size,
        Some(&vec![Individual::new(&code_string); pop_size]),
    );

    let best = gen_alg.run_genetic_algorithm(gen_num, 0.5, 0.5, 5).unwrap();

    let first_best = gen_alg
        .population_history()
        .first()
        .unwrap()
        .first()
        .unwrap();

    println!("{}", code_string);
    println!("{}\nFitness: {}", best.obj(), best.fitness());
    println!("Cycle count: {}", 20000 - best.fitness() as i32,);
    println!(
        "Original cycle count: {}",
        20000 - first_best.fitness() as i32,
    );

    //print!("{:?}\n", gen_alg.population_history());

    let duration = start_timer.elapsed();
    println!("Time elapsed: {:?}", duration);
}
