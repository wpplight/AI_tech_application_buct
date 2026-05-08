use nn::{Sequential, Linear, Loss, Module};
use tensor::Tensor;
use draw::{DrawMode, PlotConfig, plot};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║                 任务1: 多元线性回归 y = x1 + x2         ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    let mut rng = rand::rng();
    
    let num_train = 100;
    let num_val = 50;
    let num_test = 50;
    
    let (x_train, y_train) = generate_linear_data(&mut rng, num_train, 0.0, 10.0);
    let (x_val, y_val) = generate_linear_data(&mut rng, num_val, 0.0, 10.0);
    let (x_test, y_test) = generate_linear_data(&mut rng, num_test, 10.0, 20.0);
    
    let mut model = Sequential::new();
    model.add(Linear::new(2, 1));
    
    let mut loss_fn = Loss::mse();
    let lr = 0.01;
    let epochs = 500;
    
    let mut train_losses = Vec::with_capacity(epochs / 10 + 1);
    let mut val_losses = Vec::with_capacity(epochs / 10 + 1);
    
    for epoch in 0..epochs {
        for i in 0..num_train {
            let x = Tensor::build(x_train[i].clone(), vec![1, 2])?;
            let y = Tensor::build(vec![y_train[i]], vec![1, 1])?;
            
            let pred = model.forward(&x);
            let loss_result = loss_fn.criterion(&pred, &y);
            model.backward(&loss_result);
            model.update(lr);
        }
        
        if epoch % 10 == 0 {
            let train_loss = compute_loss(&mut model, &mut loss_fn, &x_train, &y_train);
            let val_loss = compute_loss(&mut model, &mut loss_fn, &x_val, &y_val);
            train_losses.push(train_loss);
            val_losses.push(val_loss);
        }
    }
    
    let train_loss = train_losses.last().copied().unwrap_or(0.0);
    let val_loss = val_losses.last().copied().unwrap_or(0.0);
    let test_loss = compute_loss(&mut model, &mut loss_fn, &x_test, &y_test);
    
    println!("  训练集损失: {:.6}", train_loss);
    println!("  验证集损失: {:.6}", val_loss);
    println!("  测试集损失: {:.6}", test_loss);
    
    let (train_points, val_points) = create_loss_curve_points(&train_losses, &val_losses);
    
    draw::plot(
        &draw::PlotConfig::new()
            .title("线性回归 训练")
            .xlabel("Epoch")
            .ylabel("Loss")
            .show_window(false)
            .export("output/regression/linear/训练.svg"),
        &[&train_points, &val_points],
        &["训练损失", "验证损失"],
    )?;
    
    draw::plot(
        &draw::PlotConfig::new()
            .title("线性回归 猜想 (R²)")
            .xlabel("真实值")
            .ylabel("预测值")
            .dot(true)
            .show_window(false)
            .export("output/regression/linear/猜想.svg"),
        &[&create_scatter_points(&mut model, &x_val, &y_val)?],
        &["验证集预测"],
    )?;
    
    draw::plot(
        &draw::PlotConfig::new()
            .title("线性回归 泛化 (R²)")
            .xlabel("真实值")
            .ylabel("预测值")
            .dot(true)
            .show_window(false)
            .export("output/regression/linear/泛化.svg"),
        &[&create_scatter_points(&mut model, &x_test, &y_test)?],
        &["测试集预测"],
    )?;
    
    println!("\n  ✅ 已保存: output/regression/linear/训练.svg");
    println!("  ✅ 已保存: output/regression/linear/猜想.svg");
    println!("  ✅ 已保存: output/regression/linear/泛化.svg");
    
    draw_3d_surface_comparison(&mut model, 0.0, 10.0, 0.0, 10.0, 20)?;
    
    println!("  ✅ 已保存: output/regression/linear/3d_fit_comparison.svg");
    
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                 任务1 完成！                           ║");
    println!("╚════════════════════════════════════════════════════════╝\n");
    
    Ok(())
}

fn generate_linear_data(
    rng: &mut impl rand::Rng,
    num_samples: usize,
    x_min: f32,
    x_max: f32,
) -> (Vec<Vec<f32>>, Vec<f32>) {
    let mut x_data = Vec::with_capacity(num_samples);
    let mut y_data = Vec::with_capacity(num_samples);
    
    for _ in 0..num_samples {
        let x1 = rng.random_range(x_min..x_max);
        let x2 = rng.random_range(x_min..x_max);
        let y = x1 + x2;
        x_data.push(vec![x1, x2]);
        y_data.push(y);
    }
    
    (x_data, y_data)
}

fn compute_loss(
    model: &mut Sequential,
    loss_fn: &mut Loss,
    x_data: &[Vec<f32>],
    y_data: &[f32],
) -> f32 {
    let mut total_loss = 0.0;
    
    for i in 0..x_data.len() {
        let x = Tensor::build(x_data[i].clone(), vec![1, 2]).unwrap();
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
        train_points.push(i as f32 * 10.0);
        train_points.push(loss);
    }
    
    for (i, &loss) in val_losses.iter().enumerate() {
        val_points.push(i as f32 * 10.0);
        val_points.push(loss);
    }
    
    let train_tensor = Tensor::build(train_points, vec![n, 2]).unwrap();
    let val_tensor = Tensor::build(val_points, vec![n, 2]).unwrap();
    
    (train_tensor, val_tensor)
}

fn create_scatter_points(
    model: &mut Sequential,
    x_data: &[Vec<f32>],
    y_data: &[f32],
) -> Result<Tensor, Box<dyn std::error::Error>> {
    let n = x_data.len();
    let mut points = Vec::with_capacity(n * 2);
    
    for i in 0..n {
        let x = Tensor::build(x_data[i].clone(), vec![1, 2])?;
        let pred = model.forward(&x);
        points.push(y_data[i]);
        points.push(pred.data()[0]);
    }
    
    Ok(Tensor::build(points, vec![n, 2])?)
}

fn draw_3d_surface_comparison(
    model: &mut Sequential,
    x1_min: f32,
    x1_max: f32,
    x2_min: f32,
    x2_max: f32,
    resolution: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut pred_vertices: Vec<f32> = Vec::with_capacity(resolution * resolution * 3);
    let mut standard_vertices: Vec<f32> = Vec::with_capacity(resolution * resolution * 3);
    
    for i in 0..resolution {
        for j in 0..resolution {
            let x1 = x1_min + (x1_max - x1_min) * (i as f32) / (resolution as f32 - 1.0);
            let x2 = x2_min + (x2_max - x2_min) * (j as f32) / (resolution as f32 - 1.0);
            
            let input = Tensor::build(vec![x1, x2], vec![1, 2])?;
            let pred = model.forward(&input);
            let y_pred = pred.data()[0];
            let y_standard = x1 + x2;
            
            // pred平面：蓝色，稍微抬高一点
            pred_vertices.push(x1);
            pred_vertices.push(x2);
            pred_vertices.push(y_pred + 0.3);
            
            // standard平面：红色，稍微压低一点
            standard_vertices.push(x1);
            standard_vertices.push(x2);
            standard_vertices.push(y_standard - 0.3);
        }
    }
    
    let mut pred_triangles: Vec<f32> = Vec::with_capacity((resolution - 1) * (resolution - 1) * 6 * 3);
    let mut standard_triangles: Vec<f32> = Vec::with_capacity((resolution - 1) * (resolution - 1) * 6 * 3);
    
    for i in 0..resolution - 1 {
        for j in 0..resolution - 1 {
            let idx00 = (i * resolution + j) * 3;
            let idx01 = (i * resolution + (j + 1)) * 3;
            let idx10 = ((i + 1) * resolution + j) * 3;
            let idx11 = ((i + 1) * resolution + (j + 1)) * 3;
            
            pred_triangles.extend_from_slice(&[pred_vertices[idx00], pred_vertices[idx00 + 1], pred_vertices[idx00 + 2]]);
            pred_triangles.extend_from_slice(&[pred_vertices[idx10], pred_vertices[idx10 + 1], pred_vertices[idx10 + 2]]);
            pred_triangles.extend_from_slice(&[pred_vertices[idx01], pred_vertices[idx01 + 1], pred_vertices[idx01 + 2]]);
            
            pred_triangles.extend_from_slice(&[pred_vertices[idx01], pred_vertices[idx01 + 1], pred_vertices[idx01 + 2]]);
            pred_triangles.extend_from_slice(&[pred_vertices[idx10], pred_vertices[idx10 + 1], pred_vertices[idx10 + 2]]);
            pred_triangles.extend_from_slice(&[pred_vertices[idx11], pred_vertices[idx11 + 1], pred_vertices[idx11 + 2]]);
            
            standard_triangles.extend_from_slice(&[standard_vertices[idx00], standard_vertices[idx00 + 1], standard_vertices[idx00 + 2]]);
            standard_triangles.extend_from_slice(&[standard_vertices[idx10], standard_vertices[idx10 + 1], standard_vertices[idx10 + 2]]);
            standard_triangles.extend_from_slice(&[standard_vertices[idx01], standard_vertices[idx01 + 1], standard_vertices[idx01 + 2]]);
            
            standard_triangles.extend_from_slice(&[standard_vertices[idx01], standard_vertices[idx01 + 1], standard_vertices[idx01 + 2]]);
            standard_triangles.extend_from_slice(&[standard_vertices[idx10], standard_vertices[idx10 + 1], standard_vertices[idx10 + 2]]);
            standard_triangles.extend_from_slice(&[standard_vertices[idx11], standard_vertices[idx11 + 1], standard_vertices[idx11 + 2]]);
        }
    }
    
    let num_pred_vertices = pred_triangles.len() / 3;
    let pred_tensor = Tensor::build(pred_triangles, vec![num_pred_vertices, 3])?;
    
    let num_standard_vertices = standard_triangles.len() / 3;
    let standard_tensor = Tensor::build(standard_triangles, vec![num_standard_vertices, 3])?;
    
    plot(
        &PlotConfig::new()
            .title("3D 平面拟合 y = x₁ + x₂ (对比)")
            .draw_mode(DrawMode::Surface)
            .show_window(false)
            .export("output/regression/linear/3d_fit_comparison.png"),
        &[&pred_tensor, &standard_tensor],
        &["预测平面", "标准平面"],
    )?;
    
    Ok(())
}