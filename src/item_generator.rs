use item::*;
use character::Attribute;
use rand::Rng;
use rand;
use names::{Generator, Name};
use types::AttributeValue;

/// A builder like generator for items. Missing fields are filled randomly
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ItemGenerator {
    data_name: Option<String>,
    data_item_type: Option<ItemType>,
    data_influence: Option<Option<ItemInfluence>>,
    data_stack_size: Option<usize>,
    data_rarity: Option<ItemRarity>,
}

impl ItemGenerator {
    /// Constructs a new `ItemGenerator`.
    ///
    /// # Examples
    ///
    /// During contruction, empty fields will be filled with random values. The following example
    /// constructs a random legendary sword.
    ///
    /// ```
    /// # use rpg::item_generator::ItemGenerator;
    /// # use rpg::item::{ItemType, ItemRarity};
    /// let item = ItemGenerator::new()
    ///     .item_type(ItemType::WeaponSword)
    ///     .rarity(ItemRarity::Legendary)
    ///     .gen();
    /// ```
    pub fn new() -> ItemGenerator {
        ItemGenerator {
            data_name: None,
            data_item_type: None,
            data_influence: None,
            data_stack_size: None,
            data_rarity: None,
        }
    }

    /// Sets the `name` of the item
    pub fn name(mut self, name: &str) -> ItemGenerator {
        self.data_name = Some(name.to_owned());
        self
    }

    /// Sets the `item_type` of the item
    pub fn item_type(mut self, item_type: ItemType) -> ItemGenerator {
        self.data_item_type = Some(item_type);
        self
    }

    /// Sets the `influence` of the item
    pub fn influence(mut self, influence: Option<ItemInfluence>) -> ItemGenerator {
        self.data_influence = Some(influence);
        self
    }

    /// Sets the `stack_size` of the item
    pub fn stack_size(mut self, stack_size: usize) -> ItemGenerator {
        self.data_stack_size = Some(stack_size);
        self
    }

    /// Sets the `rarity` of the item
    pub fn rarity(mut self, rarity: ItemRarity) -> ItemGenerator {
        self.data_rarity = Some(rarity);
        self
    }

    /// Generates the item using the given data. Missing data will be filed randomly
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

        let influence = if let Some(ref inner_influence) = self.data_influence {
            inner_influence.clone()
        } else {
            let is_none = rand::thread_rng().gen::<bool>();
            if is_none || item_type.attributes().is_empty() {
                None
            } else {
                Some(ItemInfluence {
                    attribute: random_influence_attribute(&item_type),
                    amount: random_influence_amount(&rarity),
                })
            }
        };

        // The stacksize, the item can grow to (1 if not stackable)
        let stack_size = if let Some(ref inner_stack_size) = self.data_stack_size {
            *inner_stack_size
        } else {
            random_stack_size(&item_type)
        };

        // The name of the item
        let name = if let Some(ref inner_name) = self.data_name {
            inner_name.clone()
        } else {
            random_item_name(&item_type)
        };

        Item {
            name: name,
            item_type: item_type,
            influence: influence,
            stack_size: stack_size,
            rarity: rarity,
        }
    }
}

fn random_influence_attribute(item_type: &ItemType) -> Attribute {
    let mut attrbute_set = item_type.attributes();
    if attrbute_set.is_empty() {
        Attribute::Charisma
    } else {
        let index = rand::thread_rng().gen_range(0, attrbute_set.len());
        attrbute_set.remove(index)
    }
}

fn random_influence_amount(item_rarity: &ItemRarity) -> AttributeValue {
    let mut rng = rand::thread_rng();
    let result = match *item_rarity {
        ItemRarity::Common => rng.gen_range(-1, 10),
        ItemRarity::Uncommon => rng.gen_range(1, 50),
        ItemRarity::Rare => rng.gen_range(10, 100),
        ItemRarity::Epic => rng.gen_range(50, 250),
        ItemRarity::Legendary => rng.gen_range(100, 500),
    };

    if result == 0 { 1 } else { result }
}

fn random_item_name(item_type: &ItemType) -> String {
    match *item_type {
        ItemType::WeaponSword | ItemType::WeaponHammer | ItemType::WeaponWand => {
            random_weapon_name()
        }
        _ => Generator::with_naming(Name::Plain).next().unwrap(),
    }
}

fn random_weapon_name() -> String {
    let mut weapon_names: Vec<String> = vec!["Sword", "Boulder", "Wand", "Dagger", "Hammer",
                                             "Rifle"]
        .into_iter()
        .map(|string| String::from(string))
        .collect();

    let weapon_name = rand::thread_rng().gen_range(0, weapon_names.len());
    let weapon_name = weapon_names.remove(weapon_name);

    let mut weapon_prefixes: Vec<String> = vec!["Shiny", "Firey", "Wonderous", "Giant"]
        .into_iter()
        .map(|string| String::from(string))
        .collect();

    let weapon_prefix = rand::thread_rng().gen_range(0, weapon_prefixes.len());
    let weapon_prefix = weapon_prefixes.remove(weapon_prefix);

    let mut weapon_suffixes: Vec<String> = vec!["Nashioce",
                                                "Gobloygro",
                                                "Vuskia",
                                                "Lawhos",
                                                "Shiyle",
                                                "Steiwana",
                                                "Ashington",
                                                "Ustistan",
                                                "Plez Chium",
                                                "Staej Slua",
                                                "Ospaewana",
                                                "Wespeugua",
                                                "Cuchein",
                                                "Keflya",
                                                "Speyle",
                                                "Swainia",
                                                "Eswijan",
                                                "Uswein",
                                                "Scok Slya",
                                                "Proz Drana",
                                                "Decruecia",
                                                "Vospoydan",
                                                "Xesneau",
                                                "Japlax",
                                                "Scuecia",
                                                "Dreina",
                                                "Uswela",
                                                "Usten",
                                                "Smen Snana",
                                                "Glan Gra",
                                                "Puswaenia",
                                                "Jepraoles",
                                                "Pasla",
                                                "Ewhium",
                                                "Floulia",
                                                "Plioso",
                                                "Aplurg",
                                                "Escines",
                                                "Groyt Thington",
                                                "Fleiw Flen"]
        .into_iter()
        .map(|string| String::from(string))
        .collect();

    let weapon_suffix = rand::thread_rng().gen_range(0, weapon_suffixes.len());
    let weapon_suffix = weapon_suffixes.remove(weapon_suffix);

    format!("{} {} of {}", weapon_prefix, weapon_name, weapon_suffix)
}

fn random_item_type() -> ItemType {
    rand::thread_rng().gen::<ItemType>()
}

fn random_item_rarity() -> ItemRarity {
    rand::thread_rng().gen::<ItemRarity>()
}

fn random_stack_size(item_type: &ItemType) -> usize {
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
    use character::Attribute;
    use item::{ItemType, ItemRarity, ItemInfluence};
    use rand;
    use rand::Rng;

    #[test]
    fn builder_item_type() {
        let mut rng = rand::thread_rng();
        for _ in 0..2000 {
            let rnd_type = rng.gen::<ItemType>();

            let rnd_item = ItemGenerator::new().item_type(rnd_type.clone()).gen();

            assert_eq!(rnd_item.item_type, rnd_type);
        }
    }

    #[test]
    fn builder_name() {
        let random_name = String::from("Totally random item");

        let rnd_item = ItemGenerator::new().name(random_name.clone().as_str()).gen();

        assert_eq!(rnd_item.name, random_name);
    }

    #[test]
    fn builder_influence() {
        let influence = Some(ItemInfluence::new(Attribute::Strength, 123));
        let rnd_item = ItemGenerator::new().influence(influence).gen();

        let item_influence = rnd_item.influence.unwrap();

        assert_eq!(item_influence.attribute, Attribute::Strength);
        assert_eq!(item_influence.amount, 123);
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
