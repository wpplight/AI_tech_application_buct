mod genetic;
mod crossover;
mod selection;

pub use genetic::GeneticAlgorithm;
pub use crossover::{
    single_point_crossover, 
    two_point_crossover, 
    sbx_crossover, 
    arithmetic_crossover,
    uniform_crossover,
    CrossoverFn,
};
pub use selection::{
    tournament_selection, 
    roulette_selection,
    rank_selection,
    SelectionFn,
};

pub type FitnessFn = Box<dyn Fn(&[f32]) -> f32 + Send + Sync>;
pub type MutationFn = Box<dyn Fn(&[f32]) -> Vec<f32> + Send + Sync>;

pub mod mutation {
    use super::MutationFn;
    use rand::Rng;
    
    pub fn uniform_mutation(min: f32, max: f32) -> MutationFn {
        Box::new(move |genes: &[f32]| {
            let mut mutated = genes.to_vec();
            let idx = rand::thread_rng().gen_range(0..genes.len());
            mutated[idx] = rand::thread_rng().gen_range(min..max);
            mutated
        })
    }
    
    pub fn gaussian_mutation(sigma: f32) -> MutationFn {
        Box::new(move |genes: &[f32]| {
            let mut mutated = genes.to_vec();
            let idx = rand::thread_rng().gen_range(0..genes.len());
            let noise = rand::thread_rng().gen_range(-sigma..sigma);
            mutated[idx] += noise;
            mutated
        })
    }
    
    pub fn polynomial_mutation(eta: f64, min: f32, max: f32) -> MutationFn {
        Box::new(move |genes: &[f32]| {
            let mut mutated = genes.to_vec();
            let idx = rand::thread_rng().gen_range(0..genes.len());
            let x: f64 = genes[idx] as f64;
            
            let u: f64 = rand::thread_rng().gen_range(0.0..1.0);
            let delta: f64;
            
            if u < 0.5 {
                let delta_l: f64 = (2.0 * u).powf(1.0 / (eta + 1.0)) - 1.0;
                delta = delta_l * (x - min as f64) / (x - min as f64 + 1e-10);
            } else {
                let delta_r: f64 = 1.0 - (2.0 * (1.0 - u)).powf(1.0 / (eta + 1.0));
                delta = delta_r * (max as f64 - x) / (max as f64 - x + 1e-10);
            }
            
            mutated[idx] = (x + delta).clamp(min as f64, max as f64) as f32;
            mutated
        })
    }
}

pub const DEFAULT_POP_SIZE: usize = 100;
pub const DEFAULT_TOURNAMENT_SIZE: usize = 5;
pub const DEFAULT_MUTATION_RATE: f32 = 0.1;