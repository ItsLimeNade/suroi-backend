use rand::prelude::*;
use rand::distributions::uniform::SampleUniform;
use std::f64::consts::PI;


use crate::utils::vectors::Vec2D;

pub fn weighted_random<'a, T>(items: &'a [T], weights: &Vec<f64>) -> &'a T {
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

pub fn random_float(min: &f64, max: &f64) -> f64 {
    rand::thread_rng().gen_range(*min..*max)
}

pub fn random_int(min: &i64, max: &i64) -> i64 {
    rand::thread_rng().gen_range(*min..*max)
}

pub fn random<T: SampleUniform + Ord + Copy>(min: &T, max: &T) {
    rand::thread_rng().gen_range(*min..*max);
}

pub fn rand_bool(probability: &Option<f64>) -> bool {
    let probability = match probability {
        None => f64::from(0.5),
        Some(prob) => *prob,
    };
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

pub fn rand_vec2D(min_x: &f64, max_x: &f64, min_y: &f64,max_y: &f64) -> Vec2D {
    Vec2D {
        x: random_float(min_x, max_x),
        y: random_float(min_y, max_y)
    }
}

pub fn rand_rotation() -> f64 {
    random_float(&-PI, &PI)
}
