use event::Event;

pub trait Behaviour {
    fn handle_event(&self, event: Event) -> Event;
}
