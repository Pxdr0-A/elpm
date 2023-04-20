use data::TwoDimVec;
mod data;

fn main() {
    // if a main thread ends, all other threads end!
    let number = data::random::lcg(1_000u128);
    println!("{:?}", number);

    let results = data::random::test_lcg(10_000u128);
    println!("{:?}", results);

    let shape: [usize; 2] = [3,4];
    let mut grid = TwoDimVec::new(shape);
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
    println!("{:?}", grid);
    println!("{:?}", grid.row(&0));
    println!("{:?}", grid.elm(&1, &3));

    // testing NumericDataset
}
