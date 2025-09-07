pub mod items;

use std::{error::Error, io};
fn main() -> Result<(), Box<dyn Error>> {
    println!("It's Potion time!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Issue with input");

    match input.parse::<u32>().expect("Bad input, must be numerical.") {
        _ => (),
    }

    Ok(())
}
