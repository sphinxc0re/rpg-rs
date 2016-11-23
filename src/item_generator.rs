use item::*;
use character::AttributeType;
use rand::Rng;
use rand;
use names::{Generator, Name};

pub fn random_item() -> Item {
    let item_type = random_item_type();
    random_item_with_type(&item_type)
}

pub fn random_item_with_type(item_type: &ItemType) -> Item {
    match *item_type {
        ItemType::Consumable => random_consumable(),
        ItemType::Armor => random_armor(),
        ItemType::Weapon => random_weapon(),
    }
}

fn random_consumable() -> Item {
    let name = Generator::with_naming(Name::Plain).next().unwrap();
    let item_type = ItemType::Consumable;
    let attribute = random_attribute_by_item_type(&item_type);
    let rarity = random_item_rarity();
    let attr_imfluence = random_attribute_influence_by_rarity(&rarity);
    let stack_size = 64;

    Item {
        name: name,
        item_type: item_type,
        influences_type: attribute,
        influences_by: attr_imfluence,
        stack_size: stack_size,
        rarity: rarity,
    }
}

fn random_armor() -> Item {
    let name = Generator::with_naming(Name::Plain).next().unwrap();
    let item_type = ItemType::Armor;
    let attribute = random_attribute_by_item_type(&item_type);
    let rarity = random_item_rarity();
    let attr_imfluence = random_attribute_influence_by_rarity(&rarity);
    let stack_size = 1;

    Item {
        name: name,
        item_type: item_type,
        influences_type: attribute,
        influences_by: attr_imfluence,
        stack_size: stack_size,
        rarity: rarity,
    }
}

fn random_weapon() -> Item {
    let name = Generator::with_naming(Name::Plain).next().unwrap();
    let item_type = ItemType::Weapon;
    let attribute = random_attribute_by_item_type(&item_type);
    let rarity = random_item_rarity();
    let attr_imfluence = random_attribute_influence_by_rarity(&rarity);
    let stack_size = 1;

    Item {
        name: name,
        item_type: item_type,
        influences_type: attribute,
        influences_by: attr_imfluence,
        stack_size: stack_size,
        rarity: rarity,
    }
}

fn random_attribute_by_item_type(item_type: &ItemType) -> AttributeType {
    let mut attrbute_set = attributes_by_item_type(item_type);
    let index = rand::thread_rng().gen_range(0, attrbute_set.len());
    attrbute_set.remove(index)
}

fn random_attribute_influence_by_rarity(item_rarity: &ItemRarity) -> i64 {
    let mut rng = rand::thread_rng();
    let result = match *item_rarity {
        ItemRarity::Common => rng.gen_range(-10, 10),
        ItemRarity::Uncommon => rng.gen_range(-5, 50),
        ItemRarity::Rare => rng.gen_range(0, 100),
        ItemRarity::Epic => rng.gen_range(10, 250),
        ItemRarity::Legendary => rng.gen_range(200, 500),
    };

    if result == 0 {
        1
    } else {
        result
    }
}

fn random_item_type() -> ItemType {
    rand::thread_rng().gen::<ItemType>()
}

fn random_item_rarity() -> ItemRarity {
    rand::thread_rng().gen::<ItemRarity>()
}

fn attributes_by_item_type(item_type: &ItemType) -> Vec<AttributeType> {
    match *item_type {
        ItemType::Consumable => vec![
            AttributeType::Charisma,
            AttributeType::Constitution,
            AttributeType::Defense,
            AttributeType::Dexterity,
            AttributeType::Intelligence,
            AttributeType::Luck,
            AttributeType::Perception,
            AttributeType::Strength,
            AttributeType::Willpower,
            AttributeType::Wisdom,
        ],
        ItemType::Weapon => vec![
            AttributeType::Dexterity,
            AttributeType::Luck,
            AttributeType::Strength,
        ],
        ItemType::Armor => vec![
            AttributeType::Charisma,
            AttributeType::Constitution,
            AttributeType::Defense,
            AttributeType::Dexterity,
            AttributeType::Luck,
            AttributeType::Perception,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use item::ItemType;

    #[test]
    fn item_type() {
        let rnd_item = random_item_with_type(&ItemType::Armor);

        assert_eq!(rnd_item.item_type, ItemType::Armor);
    }
}
