mod utils;
mod tests; // Do not remove
mod typings; // I have to import it here for it to be accessible in the hitbox.rs file. Fix?

use utils::random::rand_sign;

fn main() {
    println!("{}", rand_sign())
}
