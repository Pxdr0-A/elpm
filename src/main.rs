mod data;

fn main() {
    // if a main thread ends, all other threads end!
    let number = data::random::lcg(1_000u128);
    println!("{:?}", number);

    let results = data::random::test_lcg(20_000u128);
    println!("{:?}", results);
}
