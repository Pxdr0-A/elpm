mod core;
mod draw;

use crate::core::NumericDataset;
use crate::core::model::ElipsoideModel;

const DIM: usize = 2;
const SHAPE: [usize; 2] = [100, DIM];
const CLASSES: usize = 2;
const SEED: u128 = 1;

const TOL: f64 = 0.5;

fn main() {
    // if a main thread ends, all other threads end!
    let dataset: NumericDataset;
    dataset = NumericDataset::sample(SHAPE, CLASSES, SEED);

    let (x, y, target_vec) = draw::arrange_points(&dataset, &SHAPE);
    draw::scatter_template(x, y, target_vec);

    let mut model = ElipsoideModel::new(TOL, CLASSES, DIM);
    model.fit(dataset);
}
