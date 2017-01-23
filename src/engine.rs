/// The state of the whole game
#[allow(missing_docs)]
pub struct EngineContext {
    pub maps: Vec<u32>,
    pub running: bool,
    pub invalid: bool,
}

impl EngineContext {
    /// Create a new EngineContext
    pub fn new() -> EngineContext {
        EngineContext {
            maps: Vec::new(),
            running: false,
            invalid: false,
        }
    }
}

/// The engine to run the game
pub struct Engine {
    setup: Option<Box<Fn(EngineContext) -> EngineContext>>,
    update: Option<Box<Fn(EngineContext) -> EngineContext>>,
    draw: Option<Box<Fn(EngineContext) -> EngineContext>>,
}

impl Engine {
    /// Creates a new engine
    pub fn new() -> Engine {
        Engine {
            setup: None,
            update: None,
            draw: None,
        }
    }

    /// A method to define the setup behavior
    pub fn setup<T: 'static>(&mut self, setup: T)
        where T: Fn(EngineContext) -> EngineContext
    {
        self.setup = Some(Box::new(setup));
    }

    /// A method to define the update behavior
    pub fn update<T: 'static>(&mut self, update: T)
        where T: Fn(EngineContext) -> EngineContext
    {
        self.update = Some(Box::new(update));
    }

    /// A method to define the draw behavior
    pub fn draw<T: 'static>(&mut self, draw: T)
        where T: Fn(EngineContext) -> EngineContext
    {
        self.draw = Some(Box::new(draw));
    }

    /// Start the engine
    pub fn start(&self) {
        let setup = self.setup.as_ref().unwrap();
        let update = self.update.as_ref().unwrap();
        let draw = self.draw.as_ref().unwrap();

        let mut context = EngineContext::new();
        context = setup(context);

        loop {
            if !context.running {
                break;
            }

            context = update(context);

            if context.invalid {
                context = draw(context);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow() {
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
            context
        });

        engine.draw(|context| {
            // Implement your output

            // Return the altered/non-altered context
            context
        });

        // Start the engine => run the game
        engine.start();
    }
}
