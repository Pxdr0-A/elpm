use data::TwoDimVec;
use data::NumericDataset;

mod data;
mod random;

fn main() {
    // if a main thread ends, all other threads end!
    let number = random::lcg(1_000u128);
    println!("{:?}", number);

    let results = random::test_lcg(10_000u128);
    println!("{:?}", results);

    let twodimvec_shape: [usize; 2] = [3,4];
    let mut grid = TwoDimVec::new(twodimvec_shape);
    println!("{:?}", grid);

    // testing TwoDimVec
    { // add a row
        let mut row = vec![1.0,2.0,3.0,5.0];
        grid.add_row(&mut row);
        // row will say goodbye
    }
    println!("{:?}", grid);
    { // add a row
        let mut row = vec![7.0,8.0,9.0,12.0];
        grid.add_row(&mut row);
        // row will say goodbye
    }

    { // add a row
        let mut row = vec![7.0,8.0,9.0,12.0];
        grid.add_row(&mut row);
        // row will say goodbye
    }

    println!("{:?}", grid);
    println!("{:?}", grid.row(&1));
    println!("{:?}", grid.elm(&0, &3));

    // testing NumericDataset
    const NUMBER_FEATURES: usize = 5;
    const NUMBER_DATA: usize = 5;
    let dataset_shape: [usize; 2] = [NUMBER_DATA, NUMBER_FEATURES];
    let mut dataset = NumericDataset::new(dataset_shape);

    println!("{:?}", dataset);

    { // add multiple rows
        let mut count: f64 = 0.0;
        let mut row1 = vec![1.0f64; NUMBER_FEATURES].into_iter().map(|x|
            {
                count += 1.0;
                x * count.powf(2.0)
            }
        ).collect();
        count = 0.0;
        let mut row2 = vec![1.0f64; NUMBER_FEATURES].into_iter().map(|x|
            {
                count += 1.0;
                x * count.powf(-2.0)
            }
        ).collect();
        count = 0.0;
        let mut row3 = vec![1.0f64; NUMBER_FEATURES].into_iter().map(|x|
            {
                count += 1.0;
                x * (10.0f64 - count.powf(2.0))
            }
        ).collect();

        dataset.add_row(&mut row1, &1.1f64);
        dataset.add_row(&mut row2, &2.5f64);
        dataset.add_row(&mut row3, &7.4f64);
        // rows will say goodbye
    }

    println!("{:?}", dataset);
    println!("{:?}", dataset.row(&1));
    println!("{:?}", dataset.row(&0));
}
