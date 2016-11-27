use event::Event;
use behaviour::Behaviour;

/// A simple custom behaviour for quick-fixes and testing
pub struct Custom {
    handler: Box<Fn(Event) -> Event>,
}

impl Custom {
    /// Creates a new instance of `Custom`
    pub fn new<F: 'static>(handler: F) -> Custom
        where F: Fn(Event) -> Event {
        Custom {
            handler: Box::new(handler)
        }
    }
}

impl Behaviour for Custom {
    fn handle_event(&self, event: Event) -> Event {
        (self.handler)(event)
    }
}
