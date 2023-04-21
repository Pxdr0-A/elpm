use crate::core::TwoDimVec;
use crate::core::NumericDataset;

mod core;

fn main() {
    // if a main thread ends, all other threads end!
    NumericDataset::sample([5,5], 3);
}
