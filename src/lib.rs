#![deny(missing_docs)]

//! # rpg-rs
//! A **very basic** RPG engine written in Rust
//!
//! ## The Idea
//! This library should be used to build RPGs. Sounds simple and also should be, but when building
//! an RPG from scratch, most people tent to write the same code over and over again. This library
//! aims to completely implmement most of the **very basic** elements of an RPG. This is because
//! there exists a huge diversity in terms of RPGs. There os not one definite model.

extern crate names;
extern crate rand;
extern crate rustc_serialize;
extern crate term;
#[macro_use]
extern crate prettytable;

#[macro_use]
mod macros;

mod behaviour;
mod character;
mod entity;
mod event;
mod inventory;
mod item_generator;
mod item;
mod types;
mod world;
