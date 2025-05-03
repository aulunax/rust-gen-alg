use std::time::Instant;

use gen_alg::{genalg::GenAlg, individual::dlx::Individual, *};

#[test]
fn test_dlx_gen_alg() {
    let start_timer = Instant::now();

    let mut gen_alg = GenAlg::<Individual>::new(100, None);

    let best = gen_alg.run_genetic_algorithm(100, 0.5, 0.5, 5).unwrap();

    print!("{}\n", best.obj());
    //print!("{:?}\n", gen_alg.population_history());

    let duration = start_timer.elapsed();
    println!("Time elapsed: {:?}", duration);
}
