extern crate rand;
extern crate names;

#[macro_use] mod macros;
mod character;
mod inventory;
mod item;
mod item_generator;

use character::Character;

fn main() {
    let mut character = Character::new(String::from("Sheldon"));

    for _ in 0..30 {
        character.inventory.add_item(item_generator::random_item()).ok().unwrap()
    }

    println!("{:#?}", character.inventory);
}
