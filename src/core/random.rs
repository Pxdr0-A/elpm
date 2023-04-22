pub fn lcg(seed: u128) -> (u128, f64) {
    // cc65 params
    let a: u128 = 16843009;
    let b: u128 = 3014898611;
    let m: u128 = 2u128.pow(32);
    
    let rand_num = (a*seed + b) %  (m - 1);
    let rand = (rand_num as f64)/(m as f64);
    
    (rand_num, rand)
}
    
pub fn test_lcg(seed: u128) -> (f64, f64, f64) {
    let mut max_val: f64 = 0.0;
    let mut min_val: f64 = 1.0;
    let mut avg_val: f64 = 0.0;
    { // loop scope
        let n_tests: u128 = 10_000;
        let mut rand_num_int: u128 = seed;
        let mut rand_num_float: f64;
        for _ in 0..n_tests {
            (rand_num_int, rand_num_float) = lcg(rand_num_int);
            avg_val += rand_num_float;
            if rand_num_float < max_val && rand_num_float > min_val {
                continue;
            } else if rand_num_float > max_val {
                max_val = rand_num_float;
            } else if rand_num_float < min_val {
                min_val = rand_num_float;
            }
        }
    
        avg_val = avg_val/(n_tests as f64);
        // rand_num, n_tests will say goodbye here
    }
    
    (max_val, min_val, avg_val)
}