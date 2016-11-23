extern crate rand;
extern crate names;

#[macro_use] mod macros;
mod character;
mod inventory;
mod item;
mod item_generator;

use character::*;

fn main() {
    let _dummy = Character::new(String::from("Sheldon"));
}
