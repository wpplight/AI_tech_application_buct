use draw::{plot, DrawMode, PlotConfig};
use tensor::Tensor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 1: Single line with Tensor");

    let x_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_data = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let mut points = Vec::with_capacity(10);
    for i in 0..5 {
        points.push(x_data[i]);
        points.push(y_data[i]);
    }
    let points_tensor = Tensor::build(points, vec![5, 2])?;

    let config = PlotConfig::new()
        .title("Linear Function")
        .xlabel("X")
        .ylabel("Y")
        .x_range(0.0, 6.0)
        .y_range(0.0, 12.0)
        .x_ticks(7)
        .y_ticks(7)
        .show_window(false)
        .export("output/example1_linear.svg");

    plot(&config, &[&points_tensor], &["y = 2x"])?;
    println!("Exported to example1_linear.svg\n");

    println!("Example 2: Multiple lines (y = x^2, y = 2x^2, y = 0.5x^2)");
    let x_data = vec![0.0, 1.0, 2.0, 3.0, 4.0];
    let y1_data = vec![0.0, 1.0, 4.0, 9.0, 16.0];
    let y2_data = vec![0.0, 2.0, 8.0, 18.0, 32.0];
    let y3_data = vec![0.0, 0.5, 2.0, 4.5, 8.0];

    let mut points1 = Vec::with_capacity(10);
    let mut points2 = Vec::with_capacity(10);
    let mut points3 = Vec::with_capacity(10);
    for i in 0..5 {
        points1.push(x_data[i]);
        points1.push(y1_data[i]);
        points2.push(x_data[i]);
        points2.push(y2_data[i]);
        points3.push(x_data[i]);
        points3.push(y3_data[i]);
    }

    let tensor1 = Tensor::build(points1, vec![5, 2])?;
    let tensor2 = Tensor::build(points2, vec![5, 2])?;
    let tensor3 = Tensor::build(points3, vec![5, 2])?;

    let config = PlotConfig::new()
        .title("Multiple Y Series (Same X)")
        .xlabel("X")
        .ylabel("Y")
        .show_window(false)
        .export("output/example2_multi_y.svg");

    plot(&config, &[&tensor1, &tensor2, &tensor3], &["y = x^2", "y = 2x^2", "y = 0.5x^2"])?;
    println!("Exported to example2_multi_y.svg\n");

    println!("Example 3: 3D Point Cloud");
    let points_3d = vec![
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
        1.0, 1.0, 0.0,
        1.0, 0.0, 1.0,
        0.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ];
    let points_3d_tensor = Tensor::build(points_3d, vec![8, 3])?;

    let config = PlotConfig::new()
        .title("3D Points")
        .draw_mode(DrawMode::Dot)
        .show_window(false)
        .export("output/example3_3d_points.png");

    plot(&config, &[&points_3d_tensor], &["vertices"])?;
    println!("Exported to example3_3d_points.png\n");

    println!("Example 4: 3D Line (Helix)");
    let mut helix_data = Vec::new();
    for i in 0..100 {
        let t = (i as f32) * 0.1;
        helix_data.push(t.cos());
        helix_data.push(t.sin());
        helix_data.push(t * 0.5);
    }
    let helix_tensor = Tensor::build(helix_data, vec![100, 3])?;

    let config = PlotConfig::new()
        .title("3D Helix")
        .draw_mode(DrawMode::Line)
        .show_window(false)
        .export("output/example4_3d_helix.png");

    plot(&config, &[&helix_tensor], &["helix"])?;
    println!("Exported to example4_3d_helix.png\n");

    println!("Example 5: 3D Surface (Simple Triangle Mesh)");
    let surface_data = vec![
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.5, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.5, 0.5, 0.0,
        0.5, 0.5, 1.0,
    ];
    let surface_tensor = Tensor::build(surface_data, vec![6, 3])?;

    let config = PlotConfig::new()
        .title("3D Surface")
        .draw_mode(DrawMode::Surface)
        .show_window(false)
        .export("output/example5_3d_surface.png");

    plot(&config, &[&surface_tensor], &["surface"])?;
    println!("Exported to example5_3d_surface.png\n");

    Ok(())
}
