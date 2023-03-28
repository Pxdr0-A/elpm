#[derive(Debug)]
pub struct NumericDataset<'a> {
    body: &'a[f64],
    body_shape: &'a[u32],
    target: &'a[u8]
}

impl<'a> NumericDataset<'a> {
    fn sample(n_class: &u8) -> NumericDataset<'a> {
        // defining 100 data points and 10 arbitrary features
        let empty_array = &[1.0; 100*10];
        let body_shape: &[u32] = &[100, 10];
        let target: &[u8] = &[0; 100];
        
        // apply some maps to introduce some interesting patterns in the data
        let mut index = 0;
        let add_chaos = |x: f64| {
            // implement a random number generator
            // define classes domain
            index += 1;
        };

        let body = empty_array.map(add_chaos);

        NumericDataset { body, body_shape, target }
    }
}

pub fn map_2d_array(i: &u32, j: &u32, dim: [u32 ;2]) -> u32 {
    i + j*dim[1]
}

pub fn build_sample() {
    //let dataset = NumericDataset::sample();
}