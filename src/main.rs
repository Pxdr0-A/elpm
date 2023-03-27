mod data;

fn main() {
    // if a main thread ends, all other threads end!
    data::build_sample();
}
