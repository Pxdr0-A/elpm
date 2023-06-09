use plotters::prelude::*;

const COLOR_STEPS: u8 = 50;

pub fn scatter_template(x: Vec<f64>, y: Vec<f64>, target: Vec<f64>) {
    // should probabily check if x and y have the same len()
    let mut data: Vec<(f64, f64, f64)> = Vec::with_capacity(x.len());
    for i in 0..x.len() {
        data.push((x[i], y[i], target[i]));
    }

    let root_area = BitMapBackend::new("out/data.png", (800, 600))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(-20.0..150.0, -20.0..150.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
    
    let mut r_shade: u8 = 0;
    let mut g_shade: u8 = 0;
    ctx.draw_series(data.iter().map(|point| {
        if point.2 * (COLOR_STEPS as f64) >= 255.0 {
            r_shade = 255;
            g_shade = (point.2 * (COLOR_STEPS as f64) - 255.0) as u8;
        } else {
            r_shade = (point.2 as u8) * COLOR_STEPS;
        }
        Circle::new((point.0, point.1), 5, &RGBColor(r_shade, g_shade,255))
    }))
        .unwrap();
}

pub fn arrange_points(dataset: &crate::core::NumericDataset, shape: &[usize; 2]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut x: Vec<f64> = Vec::with_capacity(shape[0]);
    let mut y: Vec<f64> = Vec::with_capacity(shape[0]);
    let mut target_vec: Vec<f64> = Vec::with_capacity(shape[0]);
    let mut point: Vec<f64>;
    let mut target: f64;
    for i in 0..shape[0] {
        // assuming two dimensions
        (point, target) = dataset.row(&i);
        x.push(point[0]);
        y.push(point[1]);
        target_vec.push(target);
    }

    (x, y, target_vec)
}