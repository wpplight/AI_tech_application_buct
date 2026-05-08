pub type CrossoverFn = Box<dyn Fn(&[f32], &[f32]) -> (Vec<f32>, Vec<f32>) + Send + Sync>;

pub fn single_point_crossover(p1: &[f32], p2: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mid = p1.len() / 2;
    let mut c1 = p1[..mid].to_vec();
    c1.extend_from_slice(&p2[mid..]);
    let mut c2 = p2[..mid].to_vec();
    c2.extend_from_slice(&p1[mid..]);
    (c1, c2)
}

pub fn two_point_crossover(p1: &[f32], p2: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let len = p1.len();
    let p1_idx = rand::random::<usize>() % len;
    let p2_idx = rand::random::<usize>() % len;
    let (start, end) = if p1_idx <= p2_idx {
        (p1_idx, p2_idx)
    } else {
        (p2_idx, p1_idx)
    };
    
    let mut c1 = p1[..start].to_vec();
    c1.extend_from_slice(&p2[start..end]);
    c1.extend_from_slice(&p1[end..]);
    
    let mut c2 = p2[..start].to_vec();
    c2.extend_from_slice(&p1[start..end]);
    c2.extend_from_slice(&p2[end..]);
    
    (c1, c2)
}

pub fn sbx_crossover(eta: f64) -> CrossoverFn {
    Box::new(move |p1, p2| {
        let mut c1 = Vec::with_capacity(p1.len());
        let mut c2 = Vec::with_capacity(p2.len());
        
        for i in 0..p1.len() {
            let x1 = p1[i] as f64;
            let x2 = p2[i] as f64;
            
            let (y1, y2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
            let u = rand::random::<f64>();
            
            let beta = if u <= 0.5 {
                (2.0 * u).powf(1.0 / (eta + 1.0))
            } else {
                (1.0 / (2.0 * (1.0 - u))).powf(1.0 / (eta + 1.0))
            };
            
            c1.push(0.5 * ((y1 + y2) - beta * (y2 - y1)) as f32);
            c2.push(0.5 * ((y1 + y2) + beta * (y2 - y1)) as f32);
        }
        
        (c1, c2)
    })
}

pub fn arithmetic_crossover(alpha: f32) -> CrossoverFn {
    Box::new(move |p1, p2| {
        let c1: Vec<f32> = p1.iter()
            .zip(p2.iter())
            .map(|(a, b)| alpha * a + (1.0 - alpha) * b)
            .collect();
        let c2: Vec<f32> = p1.iter()
            .zip(p2.iter())
            .map(|(a, b)| (1.0 - alpha) * a + alpha * b)
            .collect();
        (c1, c2)
    })
}

#[allow(dead_code)]
pub fn uniform_crossover(p_swap: f32) -> CrossoverFn {
    Box::new(move |p1, p2| {
        let mut c1 = Vec::with_capacity(p1.len());
        let mut c2 = Vec::with_capacity(p2.len());
        
        for i in 0..p1.len() {
            if rand::random::<f32>() < p_swap {
                c1.push(p2[i]);
                c2.push(p1[i]);
            } else {
                c1.push(p1[i]);
                c2.push(p2[i]);
            }
        }
        
        (c1, c2)
    })
}