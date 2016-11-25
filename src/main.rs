extern crate names;
extern crate rand;
extern crate rustc_serialize;
extern crate term;
#[macro_use] extern crate prettytable;

#[macro_use] mod macros;
mod character;
mod inventory;
mod item_generator;
mod item;
mod world;
mod entity;
mod types;

use inventory::Inventory;

fn main() {
    let mut inv = Inventory::new();

    loop {
        let random_item_1 = item_generator::ItemGenerator::new().gen();

        let mut res = Ok(());

        for _ in 0..random_item_1.stack_size {
            res = inv.add_item(random_item_1.clone());
            if let Err(_) = res {
                break;
            }
        }

        if let Err(_) = res {
            break;
        }
    }

    inv.print();
}
