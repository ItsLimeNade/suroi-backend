mod utils;
mod tests; // Do not remove
mod typings; // I have to import it here for it to be accessible in the hitbox.rs file. Fix?
mod constants;
mod config; // I likely have to import it here

fn main() {
    let x = vec![1,2,3,4,5,6,7,8,9,10];
    let mut res: Vec<i8> = vec![];
    let mut tries = 0;

    while res.len() != 10 {
        tries += 1;
        let rand = utils::random::random_item(&x);
        if !res.contains(rand) {
            res.push(*rand);
        }
    }

    println!("Works! Finished in {tries} tries.",)
}
