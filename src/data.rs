const SHAPE: [i32; 2] = [100, 10];

#[derive(Debug)]
pub struct NumericDataset<'a> {
    body: &'a[f64],
    body_shape: &'a[i32],
    target: &'a[u8]
}

impl<'a> NumericDataset<'a> {
    fn sample() -> NumericDataset<'a> {
        let body = &[1.0; 100*10];
        let body_shape = &SHAPE;
        let target: &[u8; 100] = &[1; 100];
        
        // apply some maps to introduce some interesting patterns in the data

        NumericDataset { body, body_shape, target }
    }
}

pub fn build_sample() {
    let dataset = NumericDataset::sample();
}