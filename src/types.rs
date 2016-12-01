use std::collections::HashMap;
use entity::Entity;

/// Everything that can *interact with* or *be* the value of an attribute.
pub type AttributeValue = i64;

/// The type used when handling any kind of health.
pub type Health = usize;

/// The type for gold. Used as a currency.
pub type Gold = usize;

/// A map mapping entity_ids to actual entities
pub type EntityMap = HashMap<usize, Entity>;
