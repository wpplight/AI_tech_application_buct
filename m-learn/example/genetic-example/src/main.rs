mod rastrigin_variant;
mod ackley;

fn main() {
    println!("\n");
    println!("██████████████████████████████████████████");
    println!("█                                      █");
    println!("█     遗传算法优化示例集合               █");
    println!("█                                      █");
    println!("██████████████████████████████████████████");
    println!("\n");
    
    println!("开始训练...");
    println!("\n");
    
    println!("═══════════════════════════════════════════════════════");
    println!("1/2: Rastrigin变体函数优化 (求最大值)");
    println!("═══════════════════════════════════════════════════════");
    rastrigin_variant::run();
    
    println!("\n");
    println!("═══════════════════════════════════════════════════════");
    println!("2/2: Ackley函数优化 (求最小值)");
    println!("═══════════════════════════════════════════════════════");
    ackley::run();
    
    println!("\n");
    println!("██████████████████████████████████████████");
    println!("█                                      █");
    println!("█     所有优化任务完成！                  █");
    println!("█                                      █");
    println!("██████████████████████████████████████████");
    println!("\n");
    println!("输出文件保存在 output/genetic/ 目录下:");
    println!("  - rastrigin_variant/convergence.svg");
    println!("  - ackley/convergence.svg");
    println!("\n");
}