use rand::prelude::*;
use rand::distributions::uniform::SampleUniform;
use std::f64::consts::PI;
use crate::utils::vectors::Vec2D;

/// Returns a randomly selected item from the given slice of items based on the provided weights.
///
/// # Arguments
/// * `items` - A reference to a slice of items of type T.
/// * `weights` - A reference to a vector of f64 weights corresponding to each item in the slice.
///
/// # Returns
/// A reference to the selected item from the slice.
///
pub fn weighted_random<'a, T>(items: &'a [T], weights: &[f64]) -> &'a T {
    let mut i: usize = 0;
    let mut pick: f64 = rand::thread_rng().gen::<f64>() * weights.iter().sum::<f64>();

    loop {
        pick -= weights[i];
        i += 1;
        if pick <= 0.0 {
            return &items[i-1];
        }
    }
}

pub fn random_float(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_int(min: i64, max: i64) -> i64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random<T: SampleUniform + Ord + Copy>(min: T, max: T) {
    rand::thread_rng().gen_range(min..max);
}

pub fn rand_bool(probability: Option<f64>) -> bool {
    let probability = probability.unwrap_or(0.5);
    rand::thread_rng().gen_bool(probability)
}

// I might have overcomplicated that the first time...
pub fn rand_sign() -> i8 { //IMPLEMENT ONE BIT TYPE LET'S GOOO
    if rand::thread_rng().gen_bool(0.5) {
        1
    } else {
        -1
    }
}

#[allow(non_snake_case)]
pub fn rand_vec2D(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Vec2D {
    Vec2D {
        x: random_float(min_x, max_x),
        y: random_float(min_y, max_y)
    }
}

pub fn rand_rotation() -> f64 {
    random_float(-PI, PI)
}

pub fn random_point_in_circle(pos: Vec2D, min_radius: Option<f64>, max_radius: f64 ) -> Vec2D {
    let angle = random_float(0.0, PI*2.0);
    let length = random_float(min_radius.unwrap_or(0.0), max_radius);
    Vec2D {
        x: pos.x + f64::cos(angle) * length,
        y: pos.y + f64::sin(angle) * length
    }
}
