pub mod items;

use std::{error::Error, io, collections::HashMap};
use items::items::*;
fn main() -> Result<(), Box<dyn Error>> {
    let inventory: Inventory<FnMut(&mut Potion)> = Inventory{ingredient_stock: HashMap::new()};
    println!("It's Potion time!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Issue with input");

    match input.parse::<u32>().expect("Bad input, must be numerical.") {
        _ => (),
    }

    Ok(())
}

struct Inventory<F: FnMut(&mut Potion)> {
    ingredient_stock: HashMap<Ingredient<'static, F>, u32>,
}