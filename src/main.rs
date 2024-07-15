use utils::vectors::Vec2D;

mod utils;

fn main() {

    let vec: Vec2D = Vec2D::normalize(&Vec2D::new(5.0, 10.0), &None);
    let vec2: Vec2D = Vec2D::normalize(&Vec2D::new(5.0, 10.0), &Some(Vec2D::new(1.0, 0.0)));
    let vec3: Vec2D = Vec2D::normalize(&Vec2D::new(5.0, 10.0), &Some(Vec2D::new(5.0, 3.0)));

    println!("First:{vec:?}, Second:{vec2:?}, Third:{vec3:?}")
}
