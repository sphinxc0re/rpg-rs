use item::{Item, ItemType};
use std::collections::HashMap;
use inventory::Inventory;

pub struct Character {
    name: String,
    attributes: HashMap<AttributeType, i64>,
    armor_slot_head: Option<Item>,
    armor_slot_chest: Option<Item>,
    armor_slot_legs: Option<Item>,
    armor_slot_shoes: Option<Item>,
    weapon_slot_left: Option<Item>,
    weapon_slot_right: Option<Item>,
    pub inventory: Inventory,
}

impl Character {
    pub fn new(name: String) -> Character {
        let attribute_map = Self::default_attributes();
        Character {
            name: name,
            attributes: attribute_map,
            armor_slot_head: None,
            armor_slot_chest: None,
            armor_slot_legs: None,
            armor_slot_shoes: None,
            weapon_slot_left: None,
            weapon_slot_right: None,
            inventory: Inventory::new(),
        }
    }

    pub fn update_attribute(&mut self, attribute: &AttributeType, ammount: i64) {
        let mut attr = self.attributes.get(attribute).unwrap();
        attr = &ammount
    }

    pub fn get_attribute_value(&self, attribute: &AttributeType) -> i64 {
        *self.attributes.get(attribute).unwrap()
    }

    pub fn set_armor_slot_head(&mut self, item: Option<Item>) {
        if let Some(ref inner_item) = item {
            assert_eq!(inner_item.item_type, ItemType::Armor);
        }

        self.armor_slot_head = item;
    }

    pub fn set_armor_slot_chest(&mut self, item: Option<Item>) {
        if let Some(ref inner_item) = item {
            assert_eq!(inner_item.item_type, ItemType::Armor);
        }

        self.armor_slot_chest = item;
    }

    pub fn set_armor_slot_legs(&mut self, item: Option<Item>) {
        if let Some(ref inner_item) = item {
            assert_eq!(inner_item.item_type, ItemType::Armor);
        }

        self.armor_slot_legs = item;
    }

    pub fn set_armor_slot_shoes(&mut self, item: Option<Item>) {
        if let Some(ref inner_item) = item {
            assert_eq!(inner_item.item_type, ItemType::Armor);
        }

        self.armor_slot_shoes = item;
    }

    pub fn set_weapon_slot_right(&mut self, item: Option<Item>) {
        if let Some(ref inner_item) = item {
            assert_eq!(inner_item.item_type, ItemType::Weapon);
        }

        self.weapon_slot_right = item;
    }

    pub fn set_weapon_slot_left(&mut self, item: Option<Item>) {
        if let Some(ref inner_item) = item {
            assert_eq!(inner_item.item_type, ItemType::Weapon);
        }

        self.weapon_slot_left = item;
    }

    pub fn default_attributes() -> HashMap<AttributeType, i64> {
        let mut attribute_map = HashMap::new();

        attribute_map.insert(AttributeType::Charisma, 5);
        attribute_map.insert(AttributeType::Constitution, 30);
        attribute_map.insert(AttributeType::Defense, 15);
        attribute_map.insert(AttributeType::Dexterity, 10);
        attribute_map.insert(AttributeType::Intelligence, 5);
        attribute_map.insert(AttributeType::Luck, 0);
        attribute_map.insert(AttributeType::Perception, 10);
        attribute_map.insert(AttributeType::Strength, 20);
        attribute_map.insert(AttributeType::Willpower, 15);
        attribute_map.insert(AttributeType::Wisdom, 5);

        attribute_map
    }
}

vec_enum! {
    #[derive(Clone, Eq, PartialEq, Hash, Debug)]
    pub enum AttributeType {
        Charisma,
        Constitution,
        Defense,
        Dexterity,
        Intelligence,
        Luck,
        Perception,
        Strength,
        Willpower,
        Wisdom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use item_generator;
    use item::ItemType;

    #[test]
    fn default_attributes() {
        let mut attributes = Character::default_attributes();

        assert_eq!(attributes.len(), AttributeType::as_vec().len());

        for default in AttributeType::as_vec() {
            attributes.remove(&default);
        }

        assert_eq!(attributes.len(), 0);
    }

    #[test]
    fn set_armor_slot_head() {
        let mut character = Character::new(String::from("TestCharacter"));

        assert_eq!(character.armor_slot_head, None);

        let head_piece = item_generator::random_item_with_type(&ItemType::Armor);
        let head_piece_clone = head_piece.clone();

        character.set_armor_slot_head(Some(head_piece));

        assert_eq!(character.armor_slot_head, Some(head_piece_clone));
    }

    #[test]
    fn set_armor_slot_chest() {
        let mut character = Character::new(String::from("TestCharacter"));

        assert_eq!(character.armor_slot_chest, None);

        let chest_piece = item_generator::random_item_with_type(&ItemType::Armor);
        let chest_piece_clone = chest_piece.clone();

        character.set_armor_slot_chest(Some(chest_piece));

        assert_eq!(character.armor_slot_chest, Some(chest_piece_clone));
    }

    #[test]
    fn set_armor_slot_legs() {
        let mut character = Character::new(String::from("TestCharacter"));

        assert_eq!(character.armor_slot_legs, None);

        let legs_piece = item_generator::random_item_with_type(&ItemType::Armor);
        let legs_piece_clone = legs_piece.clone();

        character.set_armor_slot_legs(Some(legs_piece));

        assert_eq!(character.armor_slot_legs, Some(legs_piece_clone));
    }

    #[test]
    fn set_armor_slot_shoes() {
        let mut character = Character::new(String::from("TestCharacter"));

        assert_eq!(character.armor_slot_shoes, None);

        let shoes_piece = item_generator::random_item_with_type(&ItemType::Armor);
        let shoes_piece_clone = shoes_piece.clone();

        character.set_armor_slot_shoes(Some(shoes_piece));

        assert_eq!(character.armor_slot_shoes, Some(shoes_piece_clone));
    }

    #[test]
    fn set_weapon_slot_right() {
        let mut character = Character::new(String::from("TestCharacter"));

        assert_eq!(character.weapon_slot_right, None);

        let weapon = item_generator::random_item_with_type(&ItemType::Weapon);
        let weapon_clone = weapon.clone();

        character.set_weapon_slot_right(Some(weapon));

        assert_eq!(character.weapon_slot_right, Some(weapon_clone));
    }

    #[test]
    fn set_weapon_slot_left() {
        let mut character = Character::new(String::from("TestCharacter"));

        assert_eq!(character.weapon_slot_left, None);

        let weapon = item_generator::random_item_with_type(&ItemType::Weapon);
        let weapon_clone = weapon.clone();

        character.set_weapon_slot_left(Some(weapon));

        assert_eq!(character.weapon_slot_left, Some(weapon_clone));
    }
}
