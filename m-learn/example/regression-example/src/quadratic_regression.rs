use nn::{Sequential, Linear, ReLU, Loss, Module};
use tensor::Tensor;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║                    任务3: 二次回归 y = x²               ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    let mut rng = rand::rng();
    
    let num_train = 500;
    let num_val = 100;
    let num_test = 100;
    
    let (x_train, y_train) = generate_quadratic_data(&mut rng, num_train, 0.0, 10.0);
    let (x_val, y_val) = generate_quadratic_data(&mut rng, num_val, 0.0, 10.0);
    let (x_test, y_test) = generate_quadratic_data(&mut rng, num_test, 10.0, 20.0);
    
    let mut model = Sequential::new();
    model.add(Linear::new(1, 32));
    model.add(ReLU::new());
    model.add(Linear::new(32, 32));
    model.add(ReLU::new());
    model.add(Linear::new(32, 1));
    
    let mut loss_fn = Loss::mse();
    let lr = 0.00001;
    let epochs = 1000;
    
    let mut train_losses = Vec::with_capacity(epochs / 50 + 1);
    let mut val_losses = Vec::with_capacity(epochs / 50 + 1);
    
    for epoch in 0..epochs {
        for i in 0..num_train {
            let x = Tensor::build(vec![x_train[i]], vec![1, 1])?;
            let y = Tensor::build(vec![y_train[i]], vec![1, 1])?;
            
            let pred = model.forward(&x);
            let loss_result = loss_fn.criterion(&pred, &y);
            model.backward(&loss_result);
            model.update(lr);
        }
        
        if epoch % 50 == 0 {
            let train_loss = compute_loss(&mut model, &mut loss_fn, &x_train, &y_train);
            let val_loss = compute_loss(&mut model, &mut loss_fn, &x_val, &y_val);
            train_losses.push(train_loss);
            val_losses.push(val_loss);
            println!("  Epoch {:4}: Train Loss = {:.6}, Val Loss = {:.6}", epoch, train_loss, val_loss);
        }
    }
    
    let train_loss = compute_loss(&mut model, &mut loss_fn, &x_train, &y_train);
    let val_loss = compute_loss(&mut model, &mut loss_fn, &x_val, &y_val);
    let test_loss = compute_loss(&mut model, &mut loss_fn, &x_test, &y_test);
    
    println!("\n  训练集损失: {:.6}", train_loss);
    println!("  验证集损失: {:.6}", val_loss);
    println!("  测试集损失: {:.6}", test_loss);
    
    let (train_points, val_points) = create_loss_curve_points(&train_losses, &val_losses);
    
    draw::plot(
        &draw::PlotConfig::new()
            .title("二次回归 训练")
            .xlabel("Epoch")
            .ylabel("Loss")
            .show_window(false)
            .export("output/regression/quadratic/训练.svg"),
        &[&train_points, &val_points],
        &["训练损失", "验证损失"],
    )?;
    
    draw::plot(
        &draw::PlotConfig::new()
            .title("二次回归 猜想 (R²)")
            .xlabel("真实值")
            .ylabel("预测值")
            .dot(true)
            .show_window(false)
            .export("output/regression/quadratic/猜想.svg"),
        &[&create_scatter_points(&mut model, &x_val, &y_val)?],
        &["验证集预测"],
    )?;
    
    draw::plot(
        &draw::PlotConfig::new()
            .title("二次回归 泛化 (R²)")
            .xlabel("真实值")
            .ylabel("预测值")
            .dot(true)
            .show_window(false)
            .export("output/regression/quadratic/泛化.svg"),
        &[&create_scatter_points(&mut model, &x_test, &y_test)?],
        &["测试集预测"],
    )?;
    
    draw_fit_curve(&mut model, 0.0, 10.0, 100)?;
    
    println!("\n  ✅ 已保存: output/二次_训练.svg");
    println!("  ✅ 已保存: output/二次_猜想.svg");
    println!("  ✅ 已保存: output/二次_泛化.svg");
    println!("  ✅ 已保存: output/regression/quadratic/fit_comparison.svg");
    
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                 任务3 完成！                           ║");
    println!("╚════════════════════════════════════════════════════════╝\n");
    
    Ok(())
}

fn generate_quadratic_data(
    rng: &mut impl rand::Rng,
    num_samples: usize,
    x_min: f32,
    x_max: f32,
) -> (Vec<f32>, Vec<f32>) {
    let mut x_data = Vec::with_capacity(num_samples);
    let mut y_data = Vec::with_capacity(num_samples);
    
    for _ in 0..num_samples {
        let x = rng.random_range(x_min..x_max);
        let y = x * x;
        x_data.push(x);
        y_data.push(y);
    }
    
    (x_data, y_data)
}

fn compute_loss(
    model: &mut Sequential,
    loss_fn: &mut Loss,
    x_data: &[f32],
    y_data: &[f32],
) -> f32 {
    let mut total_loss = 0.0;
    
    for i in 0..x_data.len() {
        let x = Tensor::build(vec![x_data[i]], vec![1, 1]).unwrap();
        let y = Tensor::build(vec![y_data[i]], vec![1, 1]).unwrap();
        
        let pred = model.forward(&x);
        let loss_result = loss_fn.criterion(&pred, &y);
        total_loss += loss_result.loss;
    }
    
    total_loss / x_data.len() as f32
}

fn create_loss_curve_points(train_losses: &[f32], val_losses: &[f32]) -> (Tensor, Tensor) {
    let n = train_losses.len();
    let mut train_points = Vec::with_capacity(n * 2);
    let mut val_points = Vec::with_capacity(n * 2);
    
    for (i, &loss) in train_losses.iter().enumerate() {
        train_points.push(i as f32 * 50.0);
        train_points.push(loss);
    }
    
    for (i, &loss) in val_losses.iter().enumerate() {
        val_points.push(i as f32 * 50.0);
        val_points.push(loss);
    }
    
    let train_tensor = Tensor::build(train_points, vec![n, 2]).unwrap();
    let val_tensor = Tensor::build(val_points, vec![n, 2]).unwrap();
    
    (train_tensor, val_tensor)
}

fn create_scatter_points(
    model: &mut Sequential,
    x_data: &[f32],
    y_data: &[f32],
) -> Result<Tensor, Box<dyn std::error::Error>> {
    let n = x_data.len();
    let mut points = Vec::with_capacity(n * 2);
    
    for i in 0..n {
        let x = Tensor::build(vec![x_data[i]], vec![1, 1])?;
        let pred = model.forward(&x);
        points.push(y_data[i]);
        points.push(pred.data()[0]);
    }
    
    Ok(Tensor::build(points, vec![n, 2])?)
}

fn draw_fit_curve(
    model: &mut Sequential,
    x_min: f32,
    x_max: f32,
    num_points: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut pred_points = Vec::with_capacity(num_points * 2);
    let mut standard_points = Vec::with_capacity(num_points * 2);
    
    for i in 0..num_points {
        let x = x_min + (x_max - x_min) * (i as f32) / (num_points as f32 - 1.0);
        let input = Tensor::build(vec![x], vec![1, 1])?;
        let pred = model.forward(&input);
        let y_pred = pred.data()[0];
        let y_standard = x * x;
        
        pred_points.push(x);
        pred_points.push(y_pred);
        
        standard_points.push(x);
        standard_points.push(y_standard);
    }
    
    let pred_tensor = Tensor::build(pred_points, vec![num_points, 2])?;
    let standard_tensor = Tensor::build(standard_points, vec![num_points, 2])?;
    
    draw::plot(
        &draw::PlotConfig::new()
            .title("二次回归 y = x² (对比)")
            .xlabel("x")
            .ylabel("y")
            .show_window(false)
            .export("output/regression/quadratic/fit_comparison.svg"),
        &[&pred_tensor, &standard_tensor],
        &["预测曲线", "标准曲线"],
    )?;
    
    Ok(())
}