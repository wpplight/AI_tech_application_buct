use rand::Rng;

use crate::{CrossoverFn, FitnessFn, SelectionFn, MutationFn, DEFAULT_POP_SIZE, DEFAULT_TOURNAMENT_SIZE};
use crate::crossover::single_point_crossover;
use crate::selection::tournament_selection;

pub type ConstraintFn = Box<dyn Fn(&mut [f32]) + Send + Sync>;

pub struct GeneticAlgorithm {
    population: Vec<f32>,
    fitness_cache: Vec<f32>,
    gene_size: usize,
    pop_size: usize,
    generation: usize,
    
    fitness_fn: FitnessFn,
    crossover_fn: CrossoverFn,
    selection_fn: SelectionFn,
    mutation_fn: Option<MutationFn>,
    
    mutation_rate: f32,
    elite_count: usize,
    elite_protect: bool,
    
    gene_bounds: Option<Vec<(f32, f32)>>,
    constraint_fn: Option<ConstraintFn>,
}

impl GeneticAlgorithm {
    pub fn new(
        gene_size: usize,
        fitness_fn: impl Fn(&[f32]) -> f32 + 'static + Send + Sync,
    ) -> Self {
        Self {
            population: vec![0.0; DEFAULT_POP_SIZE * gene_size],
            fitness_cache: vec![0.0; DEFAULT_POP_SIZE],
            gene_size,
            pop_size: DEFAULT_POP_SIZE,
            generation: 0,
            fitness_fn: Box::new(fitness_fn),
            crossover_fn: Box::new(single_point_crossover),
            selection_fn: tournament_selection(DEFAULT_TOURNAMENT_SIZE),
            mutation_fn: None,
            mutation_rate: 0.1,
            elite_count: 2,
            elite_protect: false,
            gene_bounds: None,
            constraint_fn: None,
        }
    }
    
    pub fn elite_protect(mut self, enable: bool) -> Self {
        self.elite_protect = enable;
        self
    }
    
    pub fn set_mutation(mut self, rate: f32, mutation_fn: MutationFn) -> Self {
        self.mutation_rate = rate;
        self.mutation_fn = Some(mutation_fn);
        self
    }
    
    pub fn gene_bounds(mut self, bounds: Vec<(f32, f32)>) -> Self {
        assert!(bounds.len() == self.gene_size, 
            "bounds length {} must match gene_size {}", bounds.len(), self.gene_size);
        self.gene_bounds = Some(bounds);
        self
    }
    
    pub fn uniform_bounds(mut self, min: f32, max: f32) -> Self {
        let bounds: Vec<(f32, f32)> = (0..self.gene_size)
            .map(|_| (min, max))
            .collect();
        self.gene_bounds = Some(bounds);
        self
    }
    
    pub fn constrain<F>(mut self, f: F) -> Self 
    where
        F: Fn(&mut [f32]) + 'static + Send + Sync,
    {
        self.constraint_fn = Some(Box::new(f));
        self
    }
    
    fn apply_bounds(&self, genes: &mut [f32]) {
        if let Some(ref bounds) = self.gene_bounds {
            for (i, gene) in genes.iter_mut().enumerate() {
                let (min, max) = bounds[i.min(bounds.len() - 1)];
                *gene = gene.clamp(min, max);
            }
        }
    }
    
    fn apply_constraints(&self, genes: &mut [f32]) {
        if let Some(ref f) = self.constraint_fn {
            f(genes);
        }
    }
    
    fn apply_all_constraints(&self, genes: &mut [f32]) {
        self.apply_bounds(genes);
        self.apply_constraints(genes);
    }
    
    pub fn randomize(mut self) -> Self {
        let mut rng = rand::thread_rng();
        
        if let Some(ref bounds) = self.gene_bounds {
            for i in 0..self.population.len() {
                let gene_idx = i % self.gene_size;
                let (min, max) = bounds[gene_idx];
                self.population[i] = rng.gen_range(min..max);
            }
        } else {
            for i in 0..self.population.len() {
                self.population[i] = rng.gen_range(-1.0..1.0);
            }
        }
        self
    }
    
    pub fn randomize_with_range(mut self, min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        let bounds = self.gene_bounds.clone();
        
        for i in 0..self.population.len() {
            if let Some(ref b) = bounds {
                let gene_idx = i % self.gene_size;
                let (gmin, gmax) = b[gene_idx];
                self.population[i] = rng.gen_range(gmin..gmax);
            } else {
                self.population[i] = rng.gen_range(min..max);
            }
        }
        self
    }
    
    pub fn population_size(mut self, size: usize) -> Self {
        self.pop_size = size;
        self.population = vec![0.0; size * self.gene_size];
        self.fitness_cache = vec![0.0; size];
        self
    }
    
    pub fn crossover<F>(mut self, crossover_fn: F) -> Self 
    where
        F: Fn(&[f32], &[f32]) -> (Vec<f32>, Vec<f32>) + 'static + Send + Sync,
    {
        self.crossover_fn = Box::new(crossover_fn);
        self
    }
    
    pub fn selection<F>(mut self, selection_fn: F) -> Self 
    where
        F: Fn(&[f32], usize) -> Vec<usize> + 'static + Send + Sync,
    {
        self.selection_fn = Box::new(selection_fn);
        self
    }
    
    pub fn tournament_size(mut self, k: usize) -> Self {
        self.selection_fn = tournament_selection(k);
        self
    }
    
    pub fn mutation_rate(mut self, rate: f32) -> Self {
        self.mutation_rate = rate;
        self
    }
    
    pub fn elite_count(mut self, count: usize) -> Self {
        self.elite_count = count;
        self
    }
    
    pub fn step(&mut self) {
        self.fitness_cache = self.evaluate_all_parallel();
        
        let new_population: Vec<f32>;
        
        if self.elite_protect {
            let best_idx = self.fitness_cache.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, _)| i)
                .unwrap_or(0);
            
            let best_start = best_idx * self.gene_size;
            let elite_chromosome = 
                self.population[best_start..best_start + self.gene_size].to_vec();
            
            let parent_indices = self.select_parents(&self.fitness_cache);
            let offspring = self.crossover_parallel(&parent_indices);
            let mutated = self.mutate(offspring);
            
            let mut temp_population = mutated;
            if best_idx != 0 {
                let last_start = (self.pop_size - 1) * self.gene_size;
                let last_chromosome = temp_population[last_start..last_start + self.gene_size].to_vec();
                
                for i in 0..self.gene_size {
                    temp_population[best_start + i] = elite_chromosome[i];
                }
                
                let insert_pos = (self.pop_size - 1) * self.gene_size;
                for i in 0..self.gene_size {
                    temp_population[insert_pos + i] = last_chromosome[i];
                }
            }
            
            new_population = temp_population;
        } else {
            let parent_indices = self.select_parents(&self.fitness_cache);
            let offspring = self.crossover_parallel(&parent_indices);
            let mutated = self.mutate(offspring);
            new_population = mutated;
        }
        
        if self.elite_count > 0 {
            self.keep_elite(new_population);
        } else {
            self.population = new_population;
        }
        
        self.generation += 1;
    }
    
    fn evaluate_all_parallel(&self) -> Vec<f32> {
        let pop_size = self.pop_size;
        let gene_size = self.gene_size;
        let fitness_fn = &self.fitness_fn;
        let pop_len = self.population.len();
        
        let num_chunks = optim::GlobalPool::optimal_num_chunks(pop_size);
        let chunk_size = optim::GlobalPool::parallel_chunk_size(pop_size);
        
        let results: Vec<Vec<f32>> = (0..num_chunks)
            .map(|chunk_id| {
                let start = chunk_id * chunk_size;
                if start >= pop_size {
                    return Vec::new();
                }
                let end = std::cmp::min(start + chunk_size, pop_size);
                
                let mut chunk_fitness = Vec::with_capacity(end - start);
                
                for i in start..end {
                    let gene_start = i * gene_size;
                    let gene_end = gene_start + gene_size;
                    if gene_end > pop_len {
                        break;
                    }
                    let genes = &self.population[gene_start..gene_end];
                    let fitness = fitness_fn(genes);
                    chunk_fitness.push(fitness);
                }
                
                chunk_fitness
            })
            .collect();
        
        let mut result = Vec::with_capacity(pop_size);
        for chunk in results {
            result.extend(chunk);
        }
        
        result
    }
    
    fn select_parents(&self, fitness: &[f32]) -> Vec<usize> {
        let num_to_select = self.pop_size;
        (self.selection_fn)(fitness, num_to_select)
    }
    
    fn crossover_parallel(&self, parent_indices: &[usize]) -> Vec<f32> {
        let gene_size = self.gene_size;
        let crossover_fn = &self.crossover_fn;
        let pop_size = self.pop_size;
        
        if parent_indices.len() < 2 {
            return vec![0.0; pop_size * gene_size];
        }
        
        let mut result = Vec::with_capacity(pop_size * gene_size);
        
        for i in 0..pop_size {
            let p1_idx = parent_indices[i % parent_indices.len()];
            let p2_idx = parent_indices[(i + 1) % parent_indices.len()];
            
            let p1_start = p1_idx * gene_size;
            let p2_start = p2_idx * gene_size;
            let p1 = &self.population[p1_start..p1_start + gene_size];
            let p2 = &self.population[p2_start..p2_start + gene_size];
            
            let (mut child1, _child2) = crossover_fn(p1, p2);
            
            if let Some(ref mutation_fn) = self.mutation_fn {
                if rand::thread_rng().gen_range(0.0..1.0) < self.mutation_rate {
                    child1 = mutation_fn(&child1);
                }
            }
            
            result.extend_from_slice(&child1);
        }
        
        let has_bounds = self.gene_bounds.is_some();
        let has_constraints = self.constraint_fn.is_some();
        if has_bounds || has_constraints {
            for chunk in result.chunks_mut(gene_size) {
                self.apply_all_constraints(chunk);
            }
        }
        
        result
    }
    
    fn mutate(&self, mut offspring: Vec<f32>) -> Vec<f32> {
        let mutation_rate = self.mutation_rate;
        let gene_size = self.gene_size;
        let has_bounds = self.gene_bounds.is_some();
        let has_constraints = self.constraint_fn.is_some();
        
        for i in 0..offspring.len() {
            if rand::random::<f32>() < mutation_rate {
                let noise = rand::random::<f32>() * 0.2 - 0.1;
                offspring[i] += noise;
            }
        }
        
        // Apply constraints after mutation
        if has_bounds || has_constraints {
            for chunk in offspring.chunks_mut(gene_size) {
                self.apply_all_constraints(chunk);
            }
        }
        
        offspring
    }
    
    fn keep_elite(&mut self, mut new_population: Vec<f32>) {
        let elite_size = self.elite_count.min(self.pop_size);
        let gene_size = self.gene_size;
        let pop_size = self.pop_size;
        
        let mut indices: Vec<usize> = (0..pop_size).collect();
        indices.sort_by(|&a, &b| {
            self.fitness_cache[a]
                .partial_cmp(&self.fitness_cache[b])
                .unwrap()
        });
        
        let new_pop_chromosomes = new_population.len() / gene_size;
        let max_elite = elite_size.min(new_pop_chromosomes);
        
        for i in 0..max_elite {
            let worst_idx = new_pop_chromosomes - 1 - i;
            let worst_start = worst_idx * gene_size;
            let elite_start = indices[i] * gene_size;
            
            for j in 0..gene_size {
                new_population[worst_start + j] = self.population[elite_start + j];
            }
        }
        
        self.population = new_population;
    }
    
    pub fn pop_size(&self) -> usize {
        self.pop_size
    }
    
    pub fn gene_size(&self) -> usize {
        self.gene_size
    }
    
    pub fn fitness_cache(&self) -> &[f32] {
        &self.fitness_cache
    }
    
    pub fn get_fitness_sorted(&self) -> Vec<f32> {
        let mut sorted = self.fitness_cache.clone();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        sorted
    }
    
    pub fn best_chromosome(&self) -> Vec<f32> {
        if self.fitness_cache.is_empty() || self.population.is_empty() {
            return vec![0.0; self.gene_size];
        }
        
        let max_idx = self.population.len() / self.gene_size;
        
        let best_idx = self.fitness_cache.iter()
            .enumerate()
            .take(max_idx)
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        
        let start = best_idx * self.gene_size;
        let end = (start + self.gene_size).min(self.population.len());
        self.population[start..end].to_vec()
    }
    
    pub fn best_fitness(&self) -> f32 {
        self.fitness_cache.iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b))
    }
    
    pub fn average_fitness(&self) -> f32 {
        self.fitness_cache.iter().sum::<f32>() / self.pop_size as f32
    }
    
    pub fn generation(&self) -> usize {
        self.generation
    }
    
    pub fn population(&self) -> &[f32] {
        &self.population
    }
    
    pub fn get_chromosome(&self, idx: usize) -> Vec<f32> {
        let start = idx * self.gene_size;
        self.population[start..start + self.gene_size].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create() {
        let fitness_fn = |genes: &[f32]| genes.iter().sum();
        let ga = GeneticAlgorithm::new(10, fitness_fn);
        
        assert_eq!(ga.gene_size(), 10);
        assert_eq!(ga.pop_size(), 100);
    }
    
    #[test]
    fn test_randomize() {
        let fitness_fn = |genes: &[f32]| genes.iter().sum();
        let ga = GeneticAlgorithm::new(5, fitness_fn).randomize();
        
        let pop = ga.population();
        assert!(!pop.iter().all(|&x| x == 0.0));
    }
    
    #[test]
    fn test_step() {
        let fitness_fn = |genes: &[f32]| genes.iter().sum();
        let mut ga = GeneticAlgorithm::new(5, fitness_fn).randomize();
        
        let fitness_before = ga.best_fitness();
        ga.step();
        let fitness_after = ga.best_fitness();
        
        println!("Before: {}, After: {}", fitness_before, fitness_after);
    }
}