use character::Attribute;
use rand::{Rand, Rng};
use types::AttributeValue;

/// An item
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Item {
    /// The name of the item
    pub name: String,
    /// The type of the item
    pub item_type: ItemType,
    /// The influence of the item (optional)
    pub influence: Option<ItemInfluence>,
    /// The stack size of the item
    pub stack_size: usize,
    /// The rarity of the item
    pub rarity: ItemRarity,
}

impl Item {
    /// Returns `true` if the item can be equipped
    pub fn can_be_equipped(&self) -> bool {
        let equipable = vec![ItemType::ArmorHead,
                             ItemType::ArmorChest,
                             ItemType::ArmorLegs,
                             ItemType::ArmorFeet,
                             ItemType::WeaponSword,
                             ItemType::WeaponWand,
                             ItemType::WeaponHammer];

        equipable.contains(&self.item_type)
    }

    /// Returns `true` if the item can be stacked
    pub fn can_be_stacked(&self) -> bool {
        self.stack_size > 1
    }
}

/// The influence an item can have on a certain attribute
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ItemInfluence {
    /// The attribute that is influenced
    pub attribute: Attribute,
    /// The amount about which the attribute is influenced
    pub amount: AttributeValue,
}

impl ItemInfluence {
    /// Creates a new `ItemInfluence` object
    pub fn new(attribute: Attribute, amount: AttributeValue) -> ItemInfluence {
        ItemInfluence {
            attribute: attribute,
            amount: amount,
        }
    }
}

/// The type of an item
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ItemType {
    /// Armor that can only be put into the `armor_slot_head` of a character
    ArmorHead,
    /// Armor that can only be put into the `armor_slot_chest` of a character
    ArmorChest,
    /// Armor that can only be put into the `armor_slot_legs` of a character
    ArmorLegs,
    /// Armor that can only be put into the `armor_slot_feet` of a character
    ArmorFeet,

    /// A potion
    ConsumablePotion,
    /// Some kind of food
    ConsumableFood,

    /// Some kind of sword
    WeaponSword,
    /// Some kind of wand
    WeaponWand,
    /// Some kind of hammer
    WeaponHammer,

    /// A usable item
    Usable,
    /// A useless prop
    Prop,
}

impl ItemType {
    /// A list of attributes an `ItemType` can influence
    pub fn attributes(&self) -> Vec<Attribute> {
        match *self {
            ItemType::ConsumableFood |
            ItemType::ConsumablePotion => {
                vec![Attribute::Charisma,
                     Attribute::Constitution,
                     Attribute::Defense,
                     Attribute::Dexterity,
                     Attribute::Intelligence,
                     Attribute::Luck,
                     Attribute::Perception,
                     Attribute::Strength,
                     Attribute::Willpower,
                     Attribute::Wisdom]
            }
            ItemType::WeaponHammer | ItemType::WeaponSword | ItemType::WeaponWand => {
                vec![Attribute::Dexterity, Attribute::Strength]
            }
            ItemType::ArmorHead | ItemType::ArmorChest | ItemType::ArmorLegs |
            ItemType::ArmorFeet => {
                vec![Attribute::Charisma,
                     Attribute::Constitution,
                     Attribute::Defense,
                     Attribute::Dexterity,
                     Attribute::Luck,
                     Attribute::Perception]
            }
            ItemType::Usable | ItemType::Prop => vec![],
        }
    }

    /// Returns `true` if the item created using this type should be stackable
    pub fn is_stackable(&self) -> bool {
        let stackable_types = vec![ItemType::ConsumableFood, ItemType::ConsumablePotion];

        stackable_types.contains(self)
    }

    /// A helper method to get an ItemType
    pub fn by_num(item_class_num: u32, item_type_num: u32) -> ItemType {
        match item_class_num {
            0...250 => {
                match item_type_num {
                    0...500 => ItemType::ConsumableFood,
                    501...1000 => ItemType::ConsumablePotion,
                    _ => ItemType::Prop,
                }
            }
            251...500 => {
                match item_type_num {
                    0...250 => ItemType::ArmorHead,
                    251...500 => ItemType::ArmorChest,
                    501...750 => ItemType::ArmorLegs,
                    751...1000 => ItemType::ArmorFeet,
                    _ => ItemType::Prop,
                }
            }
            501...750 => {
                match item_type_num {
                    0...333 => ItemType::WeaponHammer,
                    334...666 => ItemType::WeaponSword,
                    667...1000 => ItemType::WeaponWand,
                    _ => ItemType::Prop,
                }
            }
            751...1000 => {
                match item_type_num {
                    0...500 => ItemType::Usable,
                    501...1000 => ItemType::Prop,
                    _ => ItemType::Prop,
                }
            }
            _ => ItemType::Prop,
        }
    }
}

impl Rand for ItemType {
    fn rand<R: Rng>(rng: &mut R) -> ItemType {
        let item_class_num = rng.gen_range(0, 1000);
        let item_type_num = rng.gen_range(0, 1000);

        ItemType::by_num(item_class_num, item_type_num)
    }
}

/// A type defining the rarity of an item
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ItemRarity {
    /// Items are found very often
    Common,
    /// Items are still found pretty often but not that often
    Uncommon,
    /// Items are found much rarer
    Rare,
    /// Items are extremely rare
    Epic,
    /// Items are so rare, you might never find them anywhere
    Legendary,
}

impl ItemRarity {
    /// A helper method to get an ItemRarity
    pub fn by_num(item_rarity_num: u32) -> ItemRarity {
        match item_rarity_num {
            0...750 => ItemRarity::Common,
            751...917 => ItemRarity::Uncommon,
            918...972 => ItemRarity::Rare,
            873...979 => ItemRarity::Epic,
            980...1000 => ItemRarity::Legendary,
            _ => ItemRarity::Common,
        }
    }
}

impl Rand for ItemRarity {
    fn rand<R: Rng>(rng: &mut R) -> ItemRarity {
        let base = rng.gen_range(0, 1000);

        ItemRarity::by_num(base)
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

        let head_piece =
            item_generator::ItemGenerator::new().item_type(ItemType::ConsumablePotion).gen();
        assert!(!head_piece.can_be_equipped());
    }

    #[test]
    fn can_be_stacked() {
        let head_piece = item_generator::ItemGenerator::new().stack_size(4).gen();
        assert!(head_piece.can_be_stacked());

        let head_piece = item_generator::ItemGenerator::new().stack_size(1).gen();
        assert!(!head_piece.can_be_stacked());
    }

    #[test]
    fn item_rarity() {
        assert_eq!(ItemRarity::by_num(0), ItemRarity::Common);
        assert_eq!(ItemRarity::by_num(750), ItemRarity::Common);

        assert_eq!(ItemRarity::by_num(751), ItemRarity::Uncommon);
        assert_eq!(ItemRarity::by_num(917), ItemRarity::Uncommon);

        assert_eq!(ItemRarity::by_num(918), ItemRarity::Rare);
        assert_eq!(ItemRarity::by_num(972), ItemRarity::Rare);

        assert_eq!(ItemRarity::by_num(973), ItemRarity::Epic);
        assert_eq!(ItemRarity::by_num(979), ItemRarity::Epic);

        assert_eq!(ItemRarity::by_num(980), ItemRarity::Legendary);
        assert_eq!(ItemRarity::by_num(1000), ItemRarity::Legendary);
    }

    #[test]
    fn item_type() {
        for class_num in (0..1000) {
            for type_num in (0..1000) {
                ItemType::by_num(class_num, type_num);
            }
        }
    }
}
