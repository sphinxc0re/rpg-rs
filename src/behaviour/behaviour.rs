use event::Event;

/// A behaviour defining how entities react to certain events
pub trait Behaviour: BehaviourClone {
    /// Handle an event
    fn handle_event(&self, Event) -> Event;
}

/// A helper trait for cloning `Behaviour` objects
pub trait BehaviourClone {
    /// A clone method that returns a boxed behavior
    fn clone_box(&self) -> Box<Behaviour>;
}

impl<T> BehaviourClone for T
    where T: 'static + Behaviour + Clone
{
    fn clone_box(&self) -> Box<Behaviour> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Behaviour> {
    fn clone(&self) -> Box<Behaviour> {
        self.clone_box()
    }
}
