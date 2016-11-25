use types::AttributeValue;
use item::Item;
use inventory::Inventory;

/// Everything that can attack
pub trait CanAttack {
    fn attack_damage(&self) -> AttributeValue;
}

/// Everything that can be attacked
pub trait CanBeAttacked {
    fn take_damage(&mut self);
}

/// Everything that has an inventory
pub trait HasInventory<'a> {
    fn inventory(&'a self) -> &'a Inventory;
}

/// Everything that can trade or can be traded with
pub trait CanTrade<'a> : HasInventory<'a> {
    fn trade<T: CanTrade<'a>>(&mut self, item: Item, with_other: T);

    fn send<T: CanTrade<'a>>(item: Item, to_other: T);
}

pub trait CanSpeak {
    fn default_sentence() -> String;
}

/// Everything that has is hostile
pub trait IsHostile {
}

/// Everything that has is neutral
pub trait IsNeutral {
}

/// Everything that has is friendly
pub trait IsFriendly {
}
