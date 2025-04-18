pub trait Genetic {
    fn fitness(&self) -> f32;
    fn generate() -> Self;
    fn crossover(&self, other: Self) -> Self;
    fn mutate(&self) -> ();
}
