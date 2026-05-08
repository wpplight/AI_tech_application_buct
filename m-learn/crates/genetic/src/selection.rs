use rand::seq::SliceRandom;

pub type SelectionFn = Box<dyn Fn(&[f32], usize) -> Vec<usize> + Send + Sync>;

pub fn tournament_selection(k: usize) -> SelectionFn {
    Box::new(move |fitness, num_to_select| {
        let pop_size = fitness.len();
        let actual_k = k.min(pop_size);
        let mut selected = Vec::with_capacity(num_to_select);
        
        for _ in 0..num_to_select {
            let mut contestants: Vec<usize> = (0..pop_size).collect();
            contestants.shuffle(&mut rand::thread_rng());
            let contestants: Vec<usize> = contestants.into_iter().take(actual_k).collect();
            
            if contestants.is_empty() {
                continue;
            }
            
            let winner = contestants.iter()
                .max_by(|&&a, &&b| fitness[a].partial_cmp(&fitness[b]).unwrap())
                .copied()
                .unwrap_or(0);
            selected.push(winner);
        }
        
        selected
    })
}

pub fn roulette_selection() -> SelectionFn {
    Box::new(|fitness, num_to_select| {
        let total: f32 = fitness.iter().sum();
        if total == 0.0 {
            return (0..num_to_select).collect();
        }
        
        let mut selected = Vec::with_capacity(num_to_select);
        let mut remaining: Vec<usize> = (0..fitness.len()).collect();
        
        for _ in 0..num_to_select {
            if remaining.is_empty() {
                break;
            }
            
            let mut r = rand::random::<f32>() * total;
            let mut chosen_idx = remaining.len() - 1;
            
            for (i, &idx) in remaining.iter().enumerate() {
                r -= fitness[idx];
                if r <= 0.0 {
                    chosen_idx = i;
                    break;
                }
            }
            
            let selected_idx = remaining.remove(chosen_idx);
            selected.push(selected_idx);
        }
        
        selected
    })
}

#[allow(dead_code)]
pub fn rank_selection() -> SelectionFn {
    Box::new(|fitness, num_to_select| {
        let pop_size = fitness.len();
        
        let mut indices: Vec<usize> = (0..pop_size).collect();
        indices.sort_by(|&a, &b| {
            fitness[a].partial_cmp(&fitness[b]).unwrap()
        });
        
        let rank_sum: f32 = (1..=pop_size).map(|i| i as f32).sum();
        
        let mut selected = Vec::with_capacity(num_to_select);
        for _ in 0..num_to_select {
            let mut r = rand::random::<f32>() * rank_sum;
            let mut pushed = false;
            
            for (i, &idx) in indices.iter().enumerate() {
                r -= (i + 1) as f32;
                if r <= 0.0 {
                    selected.push(idx);
                    pushed = true;
                    break;
                }
            }
            
            if !pushed {
                selected.push(*indices.last().unwrap());
            }
        }
        
        selected
    })
}