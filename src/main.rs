use std::collections::HashMap;

use crate::core::TwoDimVec;
use crate::core::NumericDataset;

mod core;

fn main() {
    // if a main thread ends, all other threads end!
    let dataset: NumericDataset;
    let centers: HashMap<String, Vec<f64>>;
    (centers, dataset) = NumericDataset::sample([20,2], 2);

    println!("{:?}", centers);
    println!("{:?}", dataset);
}
