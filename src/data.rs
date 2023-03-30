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
    use std::time::Instant;

    pub fn rand() {
        let num1 = Vec::with_capacity(3);
        let num2 = Vec::with_capacity(3);
        let address1 = &num1 as *const Vec<u8>;
        let address2 = &num2 as *const Vec<u8>;
        let wait_fct1 = address1 as u128 % 37;
        let wait_fct2 = address2 as u128 % 37;

        // do a while to simulate a coin flip
        let mut n: u128 = 0;
        let init_clock = Instant::now();
        let mut new_clock: Instant = Instant::now();
        let mut duration: u128 = 0;
        loop {
            n += 1;

            duration = new_clock.duration_since(init_clock).as_nanos();
            if duration < 100_000_000u128 + wait_fct1 {
                new_clock = Instant::now();
            } else {
                break;
            }
        }

        println!("{}", n);
        prd_linear_function(&16u8, &n);
    }

    fn prd_linear_function(bits: &u8, point: &u128) {
        // define slope
        let m: f64 = 1.0/(2u32.pow(*bits as u32) as f64);
        let cycles = (*point as f64)/(*bits as f64);
        println!("{}", cycles - ((cycles as u32) as f64));
        // define period by multiples of bits
    }
}

pub fn map_2d_array(i: &u32, j: &u32, dim: [u32 ;2]) -> u32 {
    i + j*dim[1]
}

pub fn build_sample() {
    //let dataset = NumericDataset::sample();
}