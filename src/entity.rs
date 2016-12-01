use behaviour::Behaviour;
use event::Event;

/// Any non-character element
#[derive(Clone)]
pub struct Entity {
    name: String,
    behaviour: Vec<Box<Behaviour>>,
}

impl Entity {
    /// Creates a new instance of `Entity`
    pub fn new(name: &str) -> Entity {
        Entity {
            name: name.to_owned(),
            behaviour: Vec::new(),
        }
    }

    /// Adds a behaviour ot the behaviour chain of the entity
    pub fn append_behaviour<T: Behaviour + 'static>(&mut self, behaviour: T) {
        self.behaviour.push(Box::new(behaviour));
    }

    /// Sends and event to the entity
    pub fn send_event(&self, event: Event) -> Event {
        self.handle_event(event)
    }
}

impl Behaviour for Entity {
    fn handle_event(&self, event: Event) -> Event {
        let mut last_event = Event::Nothing;
        let mut first_run = true;

        for ref behaviour in &self.behaviour {
            if first_run {
                first_run = false;
                last_event = behaviour.handle_event(event.clone());
            } else {
                last_event = behaviour.handle_event(last_event);
            }
        }

        last_event
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use behaviour::DefaultResponse;
    use event::Event;

    #[test]
    fn append_behaviour() {
        let default_response = DefaultResponse::new("Response!");

        let mut entity = Entity::new("TestSubject");

        entity.append_behaviour(default_response);

        let res = entity.send_event(Event::Nothing);

        assert_eq!(res, Event::Tell("Response!".to_owned()));
    }

    #[test]
    fn clone_entity() {
        let entity = Entity::new("TestSubject");

        let entity_clone = entity.clone();

        assert_eq!(entity.name, entity_clone.name);
    }

    #[test]
    fn chained_behaviour() {
        let default_response_one = DefaultResponse::new("Response 1!");
        let default_response_two = DefaultResponse::new("Response 2!");

        let mut entity = Entity::new("TestSubject");

        entity.append_behaviour(default_response_one);
        entity.append_behaviour(default_response_two);

        let res = entity.send_event(Event::Nothing);

        assert_eq!(res, Event::Tell("Response 2!".to_owned()));
    }
}
