mod core;
mod draw;

use crate::core::NumericDataset;
use crate::core::model::EllipsoidModel;
use std::env;

const DIM: usize = 2;
const SHAPE: [usize; 2] = [100, DIM];
const CLASSES: usize = 2;
const SEED: u128 = 111;

const TOL: f64 = 0.5;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let dataset: NumericDataset;
    dataset = NumericDataset::sample(SHAPE, CLASSES, SEED);

    let mut model = EllipsoidModel::new(TOL, CLASSES, DIM);
    model.fit(dataset.clone());

    let (x, y, target_vec) = draw::arrange_points(&dataset, &SHAPE);
    draw::scatter_template(x, y, target_vec);
}
