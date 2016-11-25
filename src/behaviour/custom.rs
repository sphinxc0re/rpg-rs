use event::Event;
use behaviour::Behaviour;

pub struct Custom {
    handler: Box<Fn(Event) -> Event>,
}

impl Custom {
    pub fn new(handler: Box<Fn(Event) -> Event>) -> Custom {
        Custom {
            handler: handler
        }
    }
}

impl Behaviour for Custom {
    fn react(&self, to: Event) -> Event {
        (self.handler)(to)
    }
}
