use rand::prelude::*;

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