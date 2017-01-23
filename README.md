# rpg-rs [![Crates.io](https://img.shields.io/crates/v/rpg.svg)](https://crates.io/crates/rpg) [![Build Status](https://travis-ci.org/raven-rpg/rpg-rs.svg?branch=master)](https://travis-ci.org/raven-rpg/rpg-rs) [![Coverage Status](https://coveralls.io/repos/github/raven-rpg/rpg-rs/badge.svg?branch=master)](https://coveralls.io/github/raven-rpg/rpg-rs?branch=master) [![Documentation](https://docs.rs/rpg/badge.svg)](https://docs.rs/rpg)
Welcome traveler! Don't be frightened by that *cryptc* name above. This project aims to implement the **very basic** data and mechanics of an RPG. Of course, there are a lot of ways to interpret and implement an RPG and I guess there will always be someone who disagrees with my implementation. So feel free to contribute and share the code and if this doesn't feel right to you, create a fork and create your own better variant.

## Roadmap
The next goal for me is to let you do this:
```rust
extern crate rpg;

use rpg::{Engine, EngineContext};

fn main() {
    let mut engine = Engine::new();

    engine.setup(|context| {
        // Setup your game
        // ...
        // ..
        // .

        // Return the altered/non-altered context
        context
    });

    engine.update(|context| {
        // Implement your update mechanics

        // Return the altered/non-altered context
        EngineContext { invalid: true, ..context }
    });

    engine.draw(|context| {
        // Implement your output

        // Return the altered/non-altered context
        EngineContext { running: false, ..context }
    });

    // Start the engine => run the game
    engine.start();
}
```
