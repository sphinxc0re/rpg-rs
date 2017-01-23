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
    setup: Box<Fn(EngineContext) -> EngineContext>,
    update: Box<Fn(EngineContext) -> EngineContext>,
    draw: Box<Fn(EngineContext) -> EngineContext>,
}

impl Engine {
    /// A method to define the setup behavior
    pub fn setup<T: 'static>(&mut self, setup: T)
        where T: Fn(EngineContext) -> EngineContext
    {
        self.setup = Box::new(setup);
    }

    /// A method to define the update behavior
    pub fn update<T: 'static>(&mut self, update: T)
        where T: Fn(EngineContext) -> EngineContext
    {
        self.update = Box::new(update);
    }

    /// A method to define the draw behavior
    pub fn draw<T: 'static>(&mut self, draw: T)
        where T: Fn(EngineContext) -> EngineContext
    {
        self.draw = Box::new(draw);
    }

    /// Start the engine
    pub fn start(&self) {
        let setup = &self.setup;
        let update = &self.update;
        let draw = &self.draw;

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
