#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![forbid(unused_import_braces)]

//! # rpg-rs
//! A **very basic** RPG engine written in Rust
//!
//! ## The Idea
//! This library should be used to build RPGs. Sounds simple and also should be, but when building
//! an RPG from scratch, most people tent to write the same code over and over again. This library
//! aims to completely implmement most of the **very basic** elements of an RPG. This is because
//! there exists a huge diversity in terms of RPGs. There is not one definite model.

extern crate names;
extern crate rand;
extern crate rustc_serialize;

/// The behaviour of entities
pub mod behaviour;
/// The structures used to bulid a character
pub mod character;
/// Everything regarding entities
pub mod entity;
/// The structure of events
pub mod event;
/// The structure and mechanics of an inventory
pub mod inventory;
/// Generate random items
pub mod item_generator;
/// The structure of items
pub mod item;
/// A module for global type consitency
pub mod types;
/// Structures for saving, loading and playing a game world
pub mod world;
/// The actual engine to run the game and handle the IO
pub mod engine;

pub use engine::Engine;
