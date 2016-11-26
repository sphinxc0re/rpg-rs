use behaviour::Behaviour;
use event::Event;

pub struct Entity {
    name: String,
    behaviour: Vec<Box<Behaviour>>,
}

impl Entity {
    pub fn new(name: &str) -> Entity {
        Entity {
            name: name.to_owned(),
            behaviour: Vec::new()
        }
    }
    pub fn append_behaviour(&mut self, behaviour: Box<Behaviour>) {
        self.behaviour.push(behaviour);
    }
}

impl Behaviour for Entity {
    fn react(&self, to: Event) -> Event {
        let mut last_event = Event::Nothing;
        let mut first_run = true;

        for ref behaviour in &self.behaviour {
            if first_run {
                first_run = false;
                last_event = behaviour.react(to.clone());
            } else {
                last_event = behaviour.react(last_event);
            }
        }

        last_event
    }
}
