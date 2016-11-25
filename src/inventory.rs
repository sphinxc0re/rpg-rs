use item::Item;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;
use prettytable::format::Alignment;
use term::Attr;

#[derive(Debug)]
pub struct InventorySlot {
    item: Item,
    amount: usize,
}

#[derive(Debug)]
pub struct Inventory {
    contents: Vec<InventorySlot>,
    max_size: usize,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            contents: Vec::new(),
            max_size: 30,
        }
    }

    pub fn print(&self) {
        let mut max_width = 0;
        for &InventorySlot { ref item, ref amount } in &self.contents {
            max_width = if item.name.len() > max_width {
                item.name.len()
            } else {
                max_width
            };
        }


        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(row!["Index", "Name", "Amount"]);
        let mut index: usize = 0;
        for &InventorySlot { ref item, ref amount } in &self.contents {
            let mut index_cell = Cell::from(&index);
            index_cell.align(Alignment::RIGHT);

            let mut item_cell = Cell::from(&item.name);
            item_cell.style(Attr::ForegroundColor(0x00AAFF));
            table.add_row(row![index_cell, item.name, amount]);
            index += 1;
        }
        table.printstd();
    }

    /// Adds an item to the inventory. If the inventory is full, the item won't be added to the
    /// inventory and a `Err(Item)` is returned.
    pub fn add_item(&mut self, new_item: Item) -> Result<(), Item> {
        for slot in &mut self.contents {
            if slot.item == new_item {
                if slot.item.stack_size > slot.amount {
                    slot.amount += 1;
                    return Ok(());
                }
            }
        }

        if self.contents.len() < self.max_size {
            self.contents.push(InventorySlot { item: new_item, amount: 1 });
        } else {
            return Err(new_item);
        }

        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.contents.len() == self.max_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use item_generator;
    use item::ItemType;

    #[test]
    fn inventory_full() {
        let mut inv_1 = Inventory::new();

        for i in 0..40 {
            let random_item = item_generator::ItemGenerator::new().item_type(ItemType::ArmorHead).gen();
            if let Err(_) = inv_1.add_item(random_item) {
                assert!(inv_1.is_full());
            }
        }
    }

    #[test]
    fn stackability() {
        let mut inv = Inventory::new();

        let random_item_1 = item_generator::ItemGenerator::new().item_type(ItemType::ConsumablePotion).gen();
        for i in 0..random_item_1.stack_size {
            let _ = inv.add_item(random_item_1.clone());
        }

        assert_eq!(inv.contents[0].amount, random_item_1.stack_size);

        for i in 0..(random_item_1.stack_size / 4) {
            let _ = inv.add_item(random_item_1.clone());
        }

        assert_eq!(inv.contents[1].amount, random_item_1.stack_size / 4);
    }
}
