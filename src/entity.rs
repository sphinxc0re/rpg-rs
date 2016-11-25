use behaviour::Behaviour;
use event::Event;

pub struct Entity {
    behaviour: Vec<Box<Behaviour>>
}

impl Entity {
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
