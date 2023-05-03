pub fn lcg(seed: u128) -> (u128, f64) {
    // IBM C/C++ convention params
    let a: u128 = 1103515245;
    let b: u128 = 12345;
    let m: u128 = 2u128.pow(31);
    
    let rand_num = (a*seed + b) %  (m - 1);
    let rand = (rand_num as f64)/(m as f64);
    
    (rand_num, rand)
}