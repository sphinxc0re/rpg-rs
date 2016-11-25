use event::Event;

pub trait Behaviour {
    fn react(&self, to: Event) -> Event;
}
