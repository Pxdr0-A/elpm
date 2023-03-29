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

        NumericDataset { body: empty_array, body_shape, target }
    }
}

pub mod random {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn rand() {
        let num1: Vec<u32> = vec![1, 2, 3];
        let num2: Vec<u32> = vec![1, 2, 3];
        let address1 = &num1 as *const Vec<u32>;
        let address2 = &num2 as *const Vec<u32>;
        let wait_fct1 = address1 as u32;
        let wait_fct2 = address2 as u32;

        println!("{}", wait_fct1 % 37);
        println!("{}", wait_fct2 % 37);
        // let inst1 = SystemTime::now();
        // do a while to simulate a coin flip
    }

    fn prd_linear_function() {

    }
}

pub fn map_2d_array(i: &u32, j: &u32, dim: [u32 ;2]) -> u32 {
    i + j*dim[1]
}

pub fn build_sample() {
    //let dataset = NumericDataset::sample();
}