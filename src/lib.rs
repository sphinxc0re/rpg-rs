#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![forbid(unused_extern_crates)]
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

#[macro_use]
pub mod macros;

pub mod behaviour;
pub mod character;
pub mod entity;
pub mod event;
pub mod inventory;
pub mod item_generator;
pub mod item;
pub mod types;
pub mod world;
