mod utils;

use utils::vectors::Vec2D;
use utils::random::weighted_random;

fn main() {
    // let vec: Vec2D = Vec2D::new(30.0, 50.0);

    // println!("{:#?}", vec);

    let items = vec!["a", "b"];
    let weights = vec![1.0,9.0];

    println!("{:#?}", weighted_random(&items, &weights));
}

