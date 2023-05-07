use crate::core::NumericDataset;

mod core;
mod draw;

const SHAPE: [usize; 2] = [500, 2];
const CLASSES: usize = 5;
const SEED: u128 = 1;

fn main() {
    // if a main thread ends, all other threads end!
    let dataset: NumericDataset;
    dataset = NumericDataset::sample(SHAPE, CLASSES, SEED);

    let (x, y, target_vec) = draw::arrange_points(&dataset, &SHAPE);
    draw::scatter_template(x, y, target_vec);
}
