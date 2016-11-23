use item::Item;

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

    /// TODO: WRITE A TEST!!!!
    /// Adds an item to the inventory. If the inventory is full, the item won't be added to the
    /// inventory and a `Err(Item)` is returned.
    pub fn add_item(&mut self, new_item: Item) -> Result<(), Item> {
        for slot in &mut self.contents {
            if slot.item == new_item {
                if slot.item.get_max_stack_size() > slot.amount {
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
            let random_item = item_generator::random_item_with_type(&ItemType::Armor);
            if let Err(_) = inv_1.add_item(random_item) {
                assert!(inv_1.is_full());
            }
        }
    }

    #[test]
    fn stackability() {
        let mut inv = Inventory::new();

        let random_item_1 = item_generator::random_item_with_type(&ItemType::Consumable);
        for i in 0..64 {
            let _ = inv.add_item(random_item_1.clone());
        }

        assert!(!inv.is_full());
        assert_eq!(inv.contents[0].amount, 64);

        for i in 0..16 {
            let _ = inv.add_item(random_item_1.clone());
        }

        assert_eq!(inv.contents[1].amount, 16);

        let random_item_2 = item_generator::random_item_with_type(&ItemType::Consumable);
        for i in 0..64 {
            let _ = inv.add_item(random_item_2.clone());
        }

        assert!(!inv.is_full());
        assert_eq!(inv.contents[2].amount, 64);

        for i in 0..16 {
            let _ = inv.add_item(random_item_2.clone());
        }

        assert_eq!(inv.contents[3].amount, 16);
    }
}
