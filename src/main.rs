mod utils;
mod tests; // Do not remove

use utils::random::rand_sign;

fn main() {
    println!("{}", rand_sign())
}
