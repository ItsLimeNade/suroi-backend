mod utils;

fn main() {
    let min: f64 = 30.0;
    let max: f64 = 45.0;
    println!("{}",utils::random::random_float(&min, &max))
}
