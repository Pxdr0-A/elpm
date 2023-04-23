use std::collections::HashMap;

use crate::core::TwoDimVec;
use crate::core::NumericDataset;

mod core;
mod draw;

const SHAPE: [usize; 2] = [200, 2];
const CLASSES: usize = 4;

fn main() {
    // if a main thread ends, all other threads end!
    let dataset: NumericDataset;
    let centers: HashMap<String, Vec<f64>>;
    (centers, dataset) = NumericDataset::sample(SHAPE, CLASSES);

    let mut x: Vec<f64> = Vec::with_capacity(SHAPE[0]);
    let mut y: Vec<f64> = Vec::with_capacity(SHAPE[0]);
    let mut target_vec: Vec<f64> = Vec::with_capacity(SHAPE[0]);
    {
        let mut point: Vec<f64>;
        let mut target: f64;
        for i in 0..SHAPE[0] {
            // assuming two dimensions
            (point, target) = dataset.row(&i);
            x.push(point[0]);
            y.push(point[1]);
            target_vec.push(target);
        }
    }

    draw::scatter_template(x, y, target_vec);
}
