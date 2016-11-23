use character::AttributeType;
use rand::{Rand, Rng};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub influences_type: AttributeType,
    pub influences_by: i64,
    pub stack_size: usize,
    pub rarity: ItemRarity,
}

impl Item {
    pub fn can_be_equipped(&self) -> bool {
        let equipable = vec![
            ItemType::Armor,
            ItemType::Weapon,
        ];

        equipable.contains(&self.item_type)
    }

    pub fn can_be_stacked(&self) -> bool {
        self.stack_size > 1
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ItemType {
    Armor,
    Consumable,
    Weapon,
}

impl ItemType {
    pub fn attributes(&self) -> Vec<AttributeType> {
        match *self {
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

    pub fn is_stackable(&self) -> bool {
        *self == ItemType::Consumable
    }
}

impl Rand for ItemType {
    fn rand<R: Rng>(rng: &mut R) -> ItemType {
        let base = rng.gen_range(0, 1000);

        match base {
            0 ... 333 => ItemType::Consumable,
            334 ... 666 => ItemType::Armor,
            667 ... 1000 => ItemType::Weapon,
            _ => ItemType::Consumable,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rand for ItemRarity {
    fn rand<R: Rng>(rng: &mut R) -> ItemRarity {
        let base = rng.gen_range(0, 1000);

        match base {
            0 ... 750 => ItemRarity::Common,
            751 ... 917 => ItemRarity::Uncommon,
            918 ... 972 => ItemRarity::Rare,
            873 ... 959 => ItemRarity::Epic,
            960 ... 1000 => ItemRarity::Legendary,
            _ => ItemRarity::Common,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use item_generator;

    #[test]
    fn can_be_equipped() {
        let head_piece = item_generator::ItemGenerator::new().item_type(ItemType::Armor).gen();
        assert!(head_piece.can_be_equipped());

        let head_piece = item_generator::ItemGenerator::new().item_type(ItemType::Consumable).gen();
        assert!(!head_piece.can_be_equipped());
    }
}
