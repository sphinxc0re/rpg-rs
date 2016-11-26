//!

use character::Attribute;
use rand::{Rand, Rng};
use types::AttributeValue;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub influence: Option<ItemInfluence>,
    pub stack_size: usize,
    pub rarity: ItemRarity,
}

impl Item {
    pub fn can_be_equipped(&self) -> bool {
        let equipable = vec![
            ItemType::ArmorHead,
            ItemType::ArmorChest,
            ItemType::ArmorLegs,
            ItemType::ArmorFeet,
            ItemType::WeaponSword,
            ItemType::WeaponWand,
            ItemType::WeaponHammer,
        ];

        equipable.contains(&self.item_type)
    }

    pub fn can_be_stacked(&self) -> bool {
        self.stack_size > 1
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ItemInfluence {
    pub attribute: Attribute,
    pub amount: AttributeValue,
}

impl ItemInfluence {
    pub fn new(attribute: Attribute, amount: AttributeValue) -> ItemInfluence {
        ItemInfluence {
            attribute: attribute,
            amount: amount,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ItemType {
    ArmorHead,
    ArmorChest,
    ArmorLegs,
    ArmorFeet,

    ConsumablePotion,
    ConsumableFood,

    WeaponSword,
    WeaponWand,
    WeaponHammer,

    Usable,
    Prop,
}

impl ItemType {
    pub fn attributes(&self) -> Vec<Attribute> {
        match *self {
            ItemType::ConsumableFood |
            ItemType::ConsumablePotion
            => vec![
                Attribute::Charisma,
                Attribute::Constitution,
                Attribute::Defense,
                Attribute::Dexterity,
                Attribute::Intelligence,
                Attribute::Luck,
                Attribute::Perception,
                Attribute::Strength,
                Attribute::Willpower,
                Attribute::Wisdom,
            ],
            ItemType::WeaponHammer |
            ItemType::WeaponSword |
            ItemType::WeaponWand
            => vec![
                Attribute::Dexterity,
                Attribute::Strength,
            ],
            ItemType::ArmorHead |
            ItemType::ArmorChest |
            ItemType::ArmorLegs |
            ItemType::ArmorFeet
            => vec![
                Attribute::Charisma,
                Attribute::Constitution,
                Attribute::Defense,
                Attribute::Dexterity,
                Attribute::Luck,
                Attribute::Perception,
            ],
            ItemType::Usable |
            ItemType::Prop
            => vec![]
        }
    }

    pub fn is_stackable(&self) -> bool {
        let stackable_types = vec![
            ItemType::ConsumableFood,
            ItemType::ConsumablePotion,
        ];

        stackable_types.contains(self)
    }
}

impl Rand for ItemType {
    fn rand<R: Rng>(rng: &mut R) -> ItemType {
        let base = rng.gen_range(0, 1000);

        match base {
            0 ... 250 => {
                let base = rng.gen_range(0, 1000);
                match base {
                    0 ... 500 => ItemType::ConsumableFood,
                    501 ... 1000 => ItemType::ConsumablePotion,
                    _ => ItemType::Prop,
                }
            },
            251 ... 500 => {
                let base = rng.gen_range(0, 1000);
                match base {
                    0 ... 250 => ItemType::ArmorHead,
                    251 ... 500 => ItemType::ArmorChest,
                    501 ... 750 => ItemType::ArmorLegs,
                    751 ... 1000 => ItemType::ArmorFeet,
                    _ => ItemType::Prop,
                }
            },
            501 ... 750 => {
                let base = rng.gen_range(0, 1000);
                match base {
                    0 ... 333 => ItemType::WeaponHammer,
                    334 ... 666 => ItemType::WeaponSword,
                    667 ... 1000 => ItemType::WeaponWand,
                    _ => ItemType::Prop,
                }
            },
            751 ... 1000 => {
                let base = rng.gen_range(0, 1000);
                match base {
                    0 ... 500 => ItemType::Usable,
                    501 ... 1000 => ItemType::Prop,
                    _ => ItemType::Prop,
                }
            },
            _ => ItemType::Prop,
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
        let head_piece = item_generator::ItemGenerator::new().item_type(ItemType::ArmorHead).gen();
        assert!(head_piece.can_be_equipped());

        let head_piece = item_generator::ItemGenerator::new().item_type(ItemType::ConsumablePotion).gen();
        assert!(!head_piece.can_be_equipped());
    }
}
