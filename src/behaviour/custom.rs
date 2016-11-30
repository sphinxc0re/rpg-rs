use event::Event;
use behaviour::Behaviour;

/// A simple custom behaviour for quick-fixes and testing
pub struct Custom {
    handler: Box<Fn(Event) -> Event>,
}

impl Custom {
    /// Creates a new instance of `Custom`
    pub fn new<F: 'static>(handler: F) -> Custom
        where F: Fn(Event) -> Event
    {
        Custom { handler: Box::new(handler) }
    }
}

impl Behaviour for Custom {
    fn handle_event(&self, event: Event) -> Event {
        (self.handler)(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use event::Event;
    use entity::Entity;

    #[test]
    fn new_custom() {
        let custom = Custom::new(|event| Event::Nothing);

        let mut entity = Entity::new("TestEntity");

        entity.append_behaviour(Box::new(custom));

        assert_eq!(entity.send_event(Event::Nothing), Event::Nothing);
    }
}
