use item::Item;
use std::collections::HashMap;
use inventory::Inventory;
use types::{Health, AttributeValue};

#[allow(missing_docs)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum SlotType {
    Head,
    Chest,
    Legs,
    Feet,
    Arms,
}

#[allow(missing_docs)]
pub struct Character {
    pub name: String,
    pub health: Health,
    pub attributes: HashMap<Attribute, AttributeValue>,
    pub slots: HashMap<SlotType, Option<Item>>,
    pub inventory: Option<Inventory>,
}

/// A list of all possible attributes
#[allow(missing_docs)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Attribute {
    Charisma,
    Constitution,
    Defense,
    Dexterity,
    Intelligence,
    Luck,
    Perception,
    Strength,
    Willpower,
    Wisdom,
}
