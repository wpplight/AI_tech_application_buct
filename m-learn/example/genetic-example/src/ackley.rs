use genetic::{GeneticAlgorithm, sbx_crossover, mutation};
use tensor::Tensor;

fn ackley(x: f32) -> f32 {
    let pi = std::f32::consts::PI;
    x * x - 10.0 * (2.0 * pi * x).cos() + 10.0
}

pub fn run() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║           Ackley函数优化 (求最小值)              ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();
    
    println!("函数: f(x) = x² - 10cos(2πx) + 10");
    println!("范围: x ∈ [-5.12, 5.12]");
    println!("最小值: 0 (在 x≈±4.9 时)");
    println!();
    
    let fitness_fn = |genes: &[f32]| {
        let x = genes[0];
        let val = ackley(x);
        if val.is_nan() || val.is_infinite() {
            f32::MIN
        } else {
            -val
        }
    };
    
    let mut ga = GeneticAlgorithm::new(1, fitness_fn)
        .population_size(2000)
        .tournament_size(20)
        .crossover(sbx_crossover(15.0))
        .set_mutation(0.01, mutation::uniform_mutation(-5.12, 5.12))
        .uniform_bounds(-5.12, 5.12)
        .elite_protect(true)
        .randomize();
    
    let mut best_ever = ga.best_chromosome();
    let mut best_ever_fitness = ga.best_fitness();
    
    let mut history = Vec::new();
    let mut global_best = f32::MAX;
    let mut global_best_x = 0.0f32;
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
        
        let current_val = -current_fitness;
        if current_val < global_best {
            global_best = current_val;
            global_best_x = current_best[0];
        }
        
        if i % 10 == 0 || i == max_gens - 1 {
            println!(
                "Gen {:4}: 当前最小 = {:10.6}, 历史最小 = {:10.6}, x = {:8.4}",
                i,
                current_val,
                global_best,
                global_best_x
            );
        }
        
        history.push((i as f32, current_val));
    }
    
    println!();
    println!("═══════════════════════════════════════════════════");
    println!("优化完成!");
    println!("═══════════════════════════════════════════════════");
    println!("最终最小解: x = {:.6}", best_ever[0]);
    println!("函数最小值: {:.6}", -best_ever_fitness);
    
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
                        .title("遗传算法收敛曲线 (Ackley函数)")
                        .xlabel("Generation")
                        .ylabel("Function Value")
                        .y_range(y_min, y_max)
                        .show_window(false)
                        .export("output/genetic/ackley/convergence.svg"),
                    &[&tensor_points],
                    &["最小值"]
                ) {
                    Ok(_) => println!("收敛曲线已保存到 output/genetic/ackley/convergence.svg"),
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
