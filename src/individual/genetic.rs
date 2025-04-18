pub trait Genetic {
    fn fitness(&self) -> f32;
    fn generate() -> Self;
}
