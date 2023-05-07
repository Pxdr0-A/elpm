use crate::core::NumericDataset;

mod core;
mod draw;

const SHAPE: [usize; 2] = [100, 2];
const CLASSES: usize = 4;

fn main() {
    // if a main thread ends, all other threads end!
    let dataset: NumericDataset;
    dataset = NumericDataset::sample(SHAPE, CLASSES);

    let (x, y, target_vec) = draw::arrange_points(&dataset, &SHAPE);
    draw::scatter_template(x, y, target_vec);
}
