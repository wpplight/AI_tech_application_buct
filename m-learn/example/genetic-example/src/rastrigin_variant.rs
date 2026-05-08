use genetic::{GeneticAlgorithm, sbx_crossover, mutation};
use tensor::Tensor;

fn objective(x: f32, y: f32) -> f32 {
    let r2 = x * x + y * y;
    let r = r2.sqrt();
    0.5 - (r - 0.5).sin() / (1.0 + 0.001 * r2).powi(2)
}

pub fn run() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║           Rastrigin变体函数优化 (求最大值)    ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!();
    
    println!("函数: f(x,y) = 0.5 - sin(√(x²+y²) - 0.5) / (1 + 0.001(x²+y²))²");
    println!("范围: x ∈ [-50, 50], y ∈ [-50, 50]");
    println!();
    
    let fitness_fn = |genes: &[f32]| {
        let x = genes[0];
        let y = genes[1];
        objective(x, y)
    };
    
    let mut ga = GeneticAlgorithm::new(2, fitness_fn)
        .population_size(2000)
        .tournament_size(20)
        .crossover(sbx_crossover(15.0))
        .set_mutation(0.01, mutation::uniform_mutation(-50.0, 50.0))
        .uniform_bounds(-50.0, 50.0)
        .elite_protect(true)
        .randomize();
    
    let mut best_ever = ga.best_chromosome();
    let mut best_ever_fitness = ga.best_fitness();
    
    let mut history = Vec::new();
    let mut global_best = 0.0f32;
    let mut global_best_x = 0.0f32;
    let mut global_best_y = 0.0f32;
    let max_gens = 500;
    
    println!("变异机制: 开启 (1% 均匀变异)");
    println!("自然选择: 激烈模式（5人锦标赛）");
    println!("开始优化 (最大 {} 代)...", max_gens);
    println!();
    
    for i in 0..max_gens {
        ga.step();
        
        let current_best = ga.best_chromosome();
        let current_fitness = ga.best_fitness();
        
        if current_fitness > best_ever_fitness {
            best_ever = current_best.clone();
            best_ever_fitness = current_fitness;
        }
        
        if current_fitness > global_best {
            global_best = current_fitness;
            global_best_x = current_best[0];
            global_best_y = current_best[1];
        }
        
        if i % 10 == 0 || i == max_gens - 1 {
            println!(
                "Gen {:4}: 当前最大 = {:10.6}, 历史最大 = {:10.6}, x = {:8.2}, y = {:8.2}",
                i,
                current_fitness,
                global_best,
                global_best_x,
                global_best_y
            );
        }
        
        history.push((i as f32, current_fitness));
    }
    
    println!();
    println!("═══════════════════════════════════════════════════");
    println!("优化完成!");
    println!("═══════════════════════════════════════════════════");
    println!("最终最大解: x = {:.6}, y = {:.6}", best_ever[0], best_ever[1]);
    println!("函数最大值: {:.6}", best_ever_fitness);
    
    if history.len() >= 2 {
        println!();
        println!("绘制收敛曲线...");
        
        let points: Vec<f32> = history.iter()
            .flat_map(|(g, fit)| vec![*g, *fit])
            .collect();
        
        let shape = vec![history.len(), 2];
        
        let min_fitness = history.iter().map(|(_, f)| *f).fold(f32::INFINITY, f32::min);
        let max_fitness = history.iter().map(|(_, f)| *f).fold(f32::NEG_INFINITY, f32::max);
        
        let (y_min, y_max) = if max_fitness - min_fitness < 0.0001 {
            let center = (max_fitness + min_fitness) / 2.0;
            (center - 0.001, center + 0.001)
        } else {
            (min_fitness, max_fitness)
        };
        
        match Tensor::build(points, shape) {
            Ok(tensor_points) => {
                match draw::plot(
                    &draw::PlotConfig::new()
                        .title("遗传算法收敛曲线 (Rastrigin变体)")
                        .xlabel("Generation")
                        .ylabel("Function Value")
                        .y_range(y_min, y_max)
                        .show_window(false)
                        .export("output/genetic/rastrigin_variant/convergence.svg"),
                    &[&tensor_points],
                    &["最大值"]
                ) {
                    Ok(_) => println!("收敛曲线已保存到 output/genetic/rastrigin_variant/convergence.svg"),
                    Err(e) => println!("绘图失败: {:?}", e),
                }
            },
            Err(e) => println!("Tensor创建失败: {:?}", e),
        }
    } else {
        println!();
        println!("历史数据不足，跳过绘图");
    }
}
