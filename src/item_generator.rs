use item::*;
use character::AttributeType;
use rand::Rng;
use rand;
use names::{Generator, Name};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ItemGenerator {
    data_name: Option<String>,
    data_item_type: Option<ItemType>,
    data_influences_type: Option<AttributeType>,
    data_influences_by: Option<i64>,
    data_stack_size: Option<usize>,
    data_rarity: Option<ItemRarity>,
}

impl ItemGenerator {
    pub fn new() -> ItemGenerator {
        ItemGenerator {
            data_name: None,
            data_item_type: None,
            data_influences_type: None,
            data_influences_by: None,
            data_stack_size: None,
            data_rarity: None,
        }
    }

    pub fn name(mut self, name: String) -> ItemGenerator {
        self.data_name = Some(name);
        self
    }

    pub fn item_type(mut self, item_type: ItemType) -> ItemGenerator {
        self.data_item_type = Some(item_type);
        self
    }

    pub fn influences_type(mut self, influences_type: AttributeType) -> ItemGenerator {
        self.data_influences_type = Some(influences_type);
        self
    }

    pub fn influences_by(mut self, influences_by: i64) -> ItemGenerator {
        self.data_influences_by = Some(influences_by);
        self
    }

    pub fn stack_size(mut self, stack_size: usize) -> ItemGenerator {
        self.data_stack_size = Some(stack_size);
        self
    }

    pub fn rarity(mut self, rarity: ItemRarity) -> ItemGenerator {
        self.data_rarity = Some(rarity);
        self
    }

    pub fn gen(&self) -> Item {
        // The item type
        let item_type = if let Some(ref inner_item_type) = self.data_item_type {
            inner_item_type.clone()
        } else {
            random_item_type()
        };

        // The item rarity
        let rarity = if let Some(ref inner_rarity) = self.data_rarity {
            inner_rarity.clone()
        } else {
            random_item_rarity()
        };

        // The attribute influenced by the item
        let influences_type = if let Some(ref inner_influences_type) = self.data_influences_type {
            inner_influences_type.clone()
        } else {
            random_influences_type_by_item_type(&item_type)
        };

        // The amount, the item influences the attribute
        let influences_by = if let Some(ref inner_influences_by) = self.data_influences_by {
            *inner_influences_by
        } else {
            random_influences_by_by_rarity(&rarity)
        };

        // The stacksize, the item can grow to (1 if not stackable)
        let stack_size = if let Some(ref inner_stack_size) = self.data_stack_size {
            *inner_stack_size
        } else {
            random_stack_size_by_item_type(&item_type)
        };

        // The name of the item
        let name = if let Some(ref inner_name) = self.data_name {
            inner_name.clone()
        } else {
            random_item_name()
        };

        Item {
            name: name,
            item_type: item_type,
            influences_type: influences_type,
            influences_by: influences_by,
            stack_size: stack_size,
            rarity: rarity,
        }
    }
}

fn random_influences_type_by_item_type(item_type: &ItemType) -> AttributeType {
    let mut attrbute_set = item_type.attributes();
    let index = rand::thread_rng().gen_range(0, attrbute_set.len());
    attrbute_set.remove(index)
}

fn random_influences_by_by_rarity(item_rarity: &ItemRarity) -> i64 {
    let mut rng = rand::thread_rng();
    let result = match *item_rarity {
        ItemRarity::Common => rng.gen_range(-1, 10),
        ItemRarity::Uncommon => rng.gen_range(1, 50),
        ItemRarity::Rare => rng.gen_range(10, 100),
        ItemRarity::Epic => rng.gen_range(50, 250),
        ItemRarity::Legendary => rng.gen_range(100, 500),
    };

    if result == 0 {
        1
    } else {
        result
    }
}

fn random_item_name() -> String {
    Generator::with_naming(Name::Plain).next().unwrap()
}

fn random_item_type() -> ItemType {
    rand::thread_rng().gen::<ItemType>()
}

fn random_item_rarity() -> ItemRarity {
    rand::thread_rng().gen::<ItemRarity>()
}

fn random_stack_size_by_item_type(item_type: &ItemType) -> usize {
    let mut base_sizes = vec![4, 16, 64];
    if item_type.is_stackable() {
        let index = rand::thread_rng().gen_range(0, base_sizes.len());
        base_sizes.remove(index) as usize
    } else {
        1
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use item::{ItemType, ItemRarity};
    use character::AttributeType;

    #[test]
    fn builder_item_type() {
        let rnd_item = ItemGenerator::new().item_type(ItemType::Armor).gen();

        assert_eq!(rnd_item.item_type, ItemType::Armor);
    }

    #[test]
    fn builder_name() {
        let random_name = String::from("Totally random item");

        let rnd_item = ItemGenerator::new().name(random_name.clone()).gen();

        assert_eq!(rnd_item.name, random_name);
    }

    #[test]
    fn builder_influences_type() {
        let rnd_item = ItemGenerator::new().influences_type(AttributeType::Strength).gen();

        assert_eq!(rnd_item.influences_type, AttributeType::Strength);
    }

    #[test]
    fn builder_influences_by() {
        let rnd_item = ItemGenerator::new().influences_by(122).gen();

        assert_eq!(rnd_item.influences_by, 122);
    }

    #[test]
    fn builder_stack_size() {
        let rnd_item = ItemGenerator::new().stack_size(45).gen();

        assert_eq!(rnd_item.stack_size, 45);
    }

    #[test]
    fn builder_rarity() {
        let rnd_item = ItemGenerator::new().rarity(ItemRarity::Rare).gen();

        assert_eq!(rnd_item.rarity, ItemRarity::Rare);
    }
}
