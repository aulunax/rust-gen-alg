# Genetic Algorithm in Rust (WIP)
Implementation of a generic genetic algorithm in Rust

## Usage
WIP

## DLX instruction SOI algorithm ideas
- Separate the code into 4 sections: Setup, Outer Loop Top, Inner Loop, Outer Loop Bottom
- Have (mostly) unchangeable branch instructions and labels, that only move under certain conditions
- Crossover makes a child with randomly chosen sections from the parent, ex. S1 + OLT2 + IL1 + OLB2
- 3 types of mutation: Add a new instruction to a section, Replace an instruction with completely different one, Change operands of an instruction 
- Fitness determined by: Accuracy of data at specified memory location, Clock cycles taken, Branch count?, Number of specific instructions?, usage of special variables (like places in memory, ex. 0x0300 for output)
- 


## TODO:
- A lot of optimizations
- Different crossover patterns
- Roulette wheel selection
- Duplicate instruction as mutation
- Cache for individuals


## Running general tests
```sh
cargo test
```

## Running the performance test
```sh
cargo test speed --release -- --ignored --show-output > speed-test.txt
```

