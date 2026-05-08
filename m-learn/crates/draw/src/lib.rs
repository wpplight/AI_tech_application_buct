use minifb::{Key, Window, WindowOptions};
use plotters::prelude::*;
use plotters::style::{register_font, FontStyle};
use plotters::coord::Shift;
use std::sync::Once;
use tensor::Tensor;

static FONT_REGISTER: Once = Once::new();

fn ensure_font() {
    FONT_REGISTER.call_once(|| {
        let _ = register_font("sans-serif", FontStyle::Normal, dejavu::sans::regular());
        let _ = register_font("sans-serif", FontStyle::Bold, dejavu::sans::bold());
    });
}

fn rgb_to_argb_u32(rgb: &[u8], out: &mut [u32]) {
    assert!(rgb.len() >= out.len() * 3);
    for (i, pixel) in out.iter_mut().enumerate() {
        let r = rgb[i * 3] as u32;
        let g = rgb[i * 3 + 1] as u32;
        let b = rgb[i * 3 + 2] as u32;
        *pixel = 0xff_00_00_00 | (r << 16) | (g << 8) | b;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportConfig {
    pub filepath: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawMode {
    Line,
    Dot,
    Surface,
}

pub struct PlotConfig {
    pub title: String,
    pub xlabel: String,
    pub ylabel: String,
    pub zlabel: String,
    pub width: u32,
    pub height: u32,
    pub legends: Vec<String>,
    pub output_path: String,
    pub x_range: Option<(f32, f32)>,
    pub y_range: Option<(f32, f32)>,
    pub z_range: Option<(f32, f32)>,
    pub x_ticks: Option<usize>,
    pub y_ticks: Option<usize>,
    pub z_ticks: Option<usize>,
    pub draw_mode: DrawMode,
    pub svg: bool,
    pub show_window: bool,
    pub export_config: Option<ExportConfig>,
}

impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            title: "Plot".to_string(),
            xlabel: "X".to_string(),
            ylabel: "Y".to_string(),
            zlabel: "Z".to_string(),
            width: 800,
            height: 600,
            legends: vec!["Series".to_string()],
            output_path: "plot.png".to_string(),
            x_range: None,
            y_range: None,
            z_range: None,
            x_ticks: None,
            y_ticks: None,
            z_ticks: None,
            draw_mode: DrawMode::Line,
            svg: false,
            show_window: true,
            export_config: None,
        }
    }
}

impl PlotConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn xlabel(mut self, xlabel: impl Into<String>) -> Self {
        self.xlabel = xlabel.into();
        self
    }

    pub fn ylabel(mut self, ylabel: impl Into<String>) -> Self {
        self.ylabel = ylabel.into();
        self
    }

    pub fn zlabel(mut self, zlabel: impl Into<String>) -> Self {
        self.zlabel = zlabel.into();
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn legends(mut self, legends: Vec<String>) -> Self {
        self.legends = legends;
        self
    }

    pub fn output_path(mut self, path: impl Into<String>) -> Self {
        self.output_path = path.into();
        self
    }

    pub fn x_range(mut self, min: f32, max: f32) -> Self {
        self.x_range = Some((min, max));
        self
    }

    pub fn y_range(mut self, min: f32, max: f32) -> Self {
        self.y_range = Some((min, max));
        self
    }

    pub fn z_range(mut self, min: f32, max: f32) -> Self {
        self.z_range = Some((min, max));
        self
    }

    pub fn x_ticks(mut self, count: usize) -> Self {
        self.x_ticks = Some(count);
        self
    }

    pub fn y_ticks(mut self, count: usize) -> Self {
        self.y_ticks = Some(count);
        self
    }

    pub fn z_ticks(mut self, count: usize) -> Self {
        self.z_ticks = Some(count);
        self
    }

    pub fn dot(mut self, enable: bool) -> Self {
        self.draw_mode = if enable { DrawMode::Dot } else { DrawMode::Line };
        self
    }

    pub fn surface(mut self) -> Self {
        self.draw_mode = DrawMode::Surface;
        self
    }

    pub fn draw_mode(mut self, mode: DrawMode) -> Self {
        self.draw_mode = mode;
        self
    }

    pub fn svg(mut self, enable: bool) -> Self {
        self.svg = enable;
        self
    }

    pub fn show_window(mut self, show: bool) -> Self {
        self.show_window = show;
        self
    }

    pub fn export(mut self, filepath: impl Into<String>) -> Self {
        self.export_config = Some(ExportConfig {
            filepath: filepath.into(),
        });
        self
    }
}

const PALETTE: [&RGBColor; 12] = [
    &BLUE, &RED, &GREEN, &CYAN, &MAGENTA, &YELLOW,
    &RGBColor(255, 127, 0),
    &RGBColor(148, 0, 211),
    &RGBColor(0, 255, 127),
    &RGBColor(255, 69, 0),
    &RGBColor(0, 191, 255),
    &RGBColor(255, 20, 147),
];

fn get_color(idx: usize) -> &'static RGBColor {
    PALETTE[idx % PALETTE.len()]
}

pub fn plot(
    config: &PlotConfig,
    points: &[&Tensor],
    legends: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    if points.is_empty() {
        return Err("At least one point series is required".into());
    }

    let shape = points[0].shape();
    if shape.len() != 2 {
        return Err(format!("Tensor must be 2D, got shape {:?}", shape).into());
    }

    let dim = shape[1];
    if dim != 2 && dim != 3 {
        return Err(format!("Tensor dimension must be 2 or 3, got {}", dim).into());
    }

    if dim == 2 {
        plot_2d(config, points, legends)?;
    } else {
        plot_3d(config, points, legends)?;
    }

    Ok(())
}

fn plot_2d(
    config: &PlotConfig,
    points: &[&Tensor],
    legends: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut all_x: Vec<f32> = Vec::new();
    let mut all_y: Vec<f32> = Vec::new();

    for tensor in points {
        let shape = tensor.shape();
        let data = tensor.data();
        for i in 0..shape[0] {
            all_x.push(data[i * 2]);
            all_y.push(data[i * 2 + 1]);
        }
    }

    let x_range = config.x_range.unwrap_or_else(|| {
        (
            all_x.iter().cloned().fold(f32::INFINITY, f32::min),
            all_x.iter().cloned().fold(f32::NEG_INFINITY, f32::max),
        )
    });
    let y_range = config.y_range.unwrap_or_else(|| {
        (
            all_y.iter().cloned().fold(f32::INFINITY, f32::min),
            all_y.iter().cloned().fold(f32::NEG_INFINITY, f32::max),
        )
    });

    if config.show_window {
        ensure_font();
        let (w, h) = (config.width as usize, config.height as usize);
        let mut rgb_buf = vec![0u8; w * h * 3];
        {
            let root = BitMapBackend::with_buffer(&mut rgb_buf, (config.width, config.height))
                .into_drawing_area();
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .margin(10)
                .caption(&config.title, ("sans-serif", 20))
                .x_label_area_size(40)
                .y_label_area_size(60)
                .build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;

            let mut mesh = chart.configure_mesh();
            mesh.x_desc(&config.xlabel).y_desc(&config.ylabel);
            if let Some(n) = config.x_ticks {
                mesh.x_labels(n);
            }
            if let Some(n) = config.y_ticks {
                mesh.y_labels(n);
            }
            mesh.draw()?;

            for (idx, tensor) in points.iter().enumerate() {
                let color = get_color(idx);
                let label = legends
                    .get(idx)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("Series {}", idx + 1));

                let shape = tensor.shape();
                let data = tensor.data();

                if matches!(config.draw_mode, DrawMode::Dot) {
                    let fill_style = color.filled();
                    chart.draw_series(
                        (0..shape[0])
                            .map(|i| Circle::new((data[i * 2], data[i * 2 + 1]), 5, fill_style.clone())),
                    )?
                    .label(label.clone())
                    .legend(|(x_leg, y_leg)| Circle::new((x_leg, y_leg), 5, get_color(0).filled()));
                } else {
                    chart.draw_series(LineSeries::new(
                        (0..shape[0]).map(|i| (data[i * 2], data[i * 2 + 1])),
                        *color,
                    ))?
                    .label(label.clone())
                    .legend(|(x_leg, y_leg)| PathElement::new(vec![(x_leg, y_leg), (x_leg + 10, y_leg)], *get_color(0)));
                }
            }

            chart
                .configure_series_labels()
                .background_style(&WHITE)
                .border_style(&BLACK)
                .draw()?;

            root.present()?;
        }

        let mut argb_buf = vec![0u32; w * h];
        rgb_to_argb_u32(&rgb_buf, &mut argb_buf);

        let mut window = Window::new(&config.title, w, h, WindowOptions::default())
            .map_err(|e| format!("minifb window: {}", e))?;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window
                .update_with_buffer(&argb_buf, w, h)
                .map_err(|e| format!("minifb update: {}", e))?;
        }
    }

    if let Some(ref export_config) = config.export_config {
        ensure_font();
        
        std::fs::create_dir_all(
            std::path::Path::new(&export_config.filepath)
                .parent()
                .unwrap_or(std::path::Path::new("."))
        )?;
        
        let filepath = &export_config.filepath;
        let ext = std::path::Path::new(filepath)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("svg")
            .to_lowercase();
        
        let root: DrawingArea<SVGBackend, Shift>;
        if ext == "svg" {
            root = SVGBackend::new(filepath, (config.width, config.height))
                .into_drawing_area();
        } else {
            return Err(format!(
                "Unsupported image format '{}'. Only SVG is supported for 2D plots. Use .svg extension.",
                ext
            ).into());
        }

        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption(&config.title, ("sans-serif", 20))
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;

        let mut mesh = chart.configure_mesh();
        mesh.x_desc(&config.xlabel).y_desc(&config.ylabel);
        if let Some(n) = config.x_ticks {
            mesh.x_labels(n);
        }
        if let Some(n) = config.y_ticks {
            mesh.y_labels(n);
        }
        mesh.draw()?;

        for (idx, tensor) in points.iter().enumerate() {
            let color = get_color(idx);
            let label = legends
                .get(idx)
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("Series {}", idx + 1));

            let shape = tensor.shape();
            let data = tensor.data();

            if matches!(config.draw_mode, DrawMode::Dot) {
                let fill_style = color.filled();
                chart.draw_series(
                    (0..shape[0])
                        .map(|i| Circle::new((data[i * 2], data[i * 2 + 1]), 5, fill_style.clone())),
                )?
                .label(label)
                .legend(|(x_leg, y_leg)| Circle::new((x_leg, y_leg), 5, get_color(0).filled()));
            } else {
                chart.draw_series(LineSeries::new(
                    (0..shape[0]).map(|i| (data[i * 2], data[i * 2 + 1])),
                    *color,
                ))?
                .label(label)
                .legend(|(x_leg, y_leg)| PathElement::new(vec![(x_leg, y_leg), (x_leg + 10, y_leg)], *get_color(0)));
            }
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE)
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
    }

    Ok(())
}

fn plot_3d(
    config: &PlotConfig,
    points: &[&Tensor],
    _legends: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = polyscope_rs::init();

    // 设置黑色背景
    let black_background = polyscope_rs::Vec4::new(0.0, 0.0, 0.0, 1.0);
    let _ = polyscope_rs::with_context_mut(|ctx| {
        ctx.options.background_color = black_background;
        // 禁用透明度，确保两个重合平面能区分
        ctx.options.transparency_enabled = false;
    });

    // 禁用地面（我们将手动绘制坐标轴）
    let _ = polyscope_rs::with_context_mut(|ctx| {
        ctx.options.ground_plane_enabled = false;
        // 禁用SSAO以获得更纯的黑色背景
        ctx.options.ssao.enabled = false;
    });

    // 只在第一次调用时绘制坐标轴（通过检查是否已存在）
    if polyscope_rs::get_curve_network("Coordinate_Axes").is_none() {
        // 计算数据的边界
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_z = f32::NEG_INFINITY;

        for tensor in points {
            let data = tensor.data();
            let n = tensor.shape()[0];
            for i in 0..n {
                let x = data[i * 3];
                let y = data[i * 3 + 1];
                let z = data[i * 3 + 2];
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
                min_z = min_z.min(z);
                max_z = max_z.max(z);
            }
        }

        // 计算坐标轴长度（取最大值域的某个比例）
        let axis_scale = 1.1;
        let axis_length = ((max_x - min_x).max(max_y - min_y).max(max_z - min_z) * axis_scale).max(1.0);

        // 绘制坐标轴
        let origin = polyscope_rs::Vec3::new(0.0, 0.0, 0.0);
        let axis_vertices: Vec<polyscope_rs::Vec3> = vec![
            // X轴
            origin,
            polyscope_rs::Vec3::new(axis_length, 0.0, 0.0),
            // Y轴
            origin,
            polyscope_rs::Vec3::new(0.0, axis_length, 0.0),
            // Z轴
            origin,
            polyscope_rs::Vec3::new(0.0, 0.0, axis_length),
        ];

        let axis_edges: Vec<[u32; 2]> = vec![
            [0, 1],  // X轴
            [2, 3],  // Y轴
            [4, 5],  // Z轴
        ];

        let _axis_network = polyscope_rs::register_curve_network("Coordinate_Axes", axis_vertices, axis_edges);

        // 设置坐标轴为白色
        let _ = polyscope_rs::with_curve_network("Coordinate_Axes", |network: &mut polyscope_rs::CurveNetwork| {
            let white_color = polyscope_rs::Vec3::new(1.0, 1.0, 1.0);
            network.set_color(white_color);
        });
    }

    for (idx, tensor) in points.iter().enumerate() {
        let data = tensor.data();
        let n = tensor.shape()[0];
        let mut vertices = Vec::with_capacity(n);
        for i in 0..n {
            vertices.push(polyscope_rs::Vec3::new(data[i * 3], data[i * 3 + 1], data[i * 3 + 2]));
        }
        let name = format!("{}_{}", config.title, idx);

        match config.draw_mode {
            DrawMode::Dot => {
                polyscope_rs::register_point_cloud(&name, vertices);
            }
            DrawMode::Line => {
                let edges: Vec<[u32; 2]> = (0..n - 1).map(|i| [i as u32, i as u32 + 1]).collect();
                polyscope_rs::register_curve_network(&name, vertices, edges);
            }
            DrawMode::Surface => {
                let num_tris = n / 3;
                let mut faces = Vec::with_capacity(num_tris);
                for i in 0..num_tris {
                    let base = i as u32 * 3;
                    faces.push([base, base + 1, base + 2]);
                }
                polyscope_rs::register_surface_mesh(&name, vertices, faces);
                
                // 为每个表面设置不同的颜色，使用完全饱和的纯色
                let _ = polyscope_rs::with_surface_mesh(&name, |mesh: &mut polyscope_rs::SurfaceMesh| {
                    // 纯色，无透明度，确保两个重合平面能区分
                    // 第一个平面：纯蓝色，第二个平面：纯红色
                    let colors = [
                        (0.0, 0.0, 1.0),    // 纯蓝色 (idx 0)
                        (1.0, 0.0, 0.0),    // 纯红色 (idx 1)
                        (0.0, 1.0, 0.0),    // 绿色 (idx 2)
                        (1.0, 1.0, 0.0),    // 黄色 (idx 3)
                    ];
                    let edge_colors = [
                        (0.0, 0.0, 0.8),  // 深蓝色边缘 (idx 0)
                        (0.8, 0.0, 0.0),  // 深红色边缘 (idx 1)
                        (0.0, 0.8, 0.0),  // 深绿色边缘 (idx 2)
                        (0.8, 0.6, 0.0),  // 深黄色边缘 (idx 3)
                    ];
                    
                    let (r, g, b) = colors[idx % colors.len()];
                    let (er, eg, eb) = edge_colors[idx % edge_colors.len()];
                    
                    // 设置完全不透明的表面颜色
                    mesh.set_surface_color(polyscope_rs::Vec3::new(r, g, b));
                    
                    // 启用边缘线显示
                    mesh.set_show_edges(true);
                    mesh.set_edge_color(polyscope_rs::Vec3::new(er, eg, eb));
                    mesh.set_edge_width(2.0);
                    
                    // 设置backface颜色为相同颜色
                    mesh.set_backface_color(polyscope_rs::Vec3::new(r, g, b));
                });
            }
        }
    }

    if let Some(ref export_config) = config.export_config {
        let path = &export_config.filepath;
        
        std::fs::create_dir_all(
            std::path::Path::new(path)
                .parent()
                .unwrap_or(std::path::Path::new("."))
        )?;
        
        polyscope_rs::render_to_file(path, config.width as u32, config.height as u32)?;
        println!("Exported 3D plot to {}", path);
    } else {
        polyscope_rs::show();
    }

    Ok(())
}
