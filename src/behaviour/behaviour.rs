use event::Event;

/// A behaviour defining how entities react to certain events
pub trait Behaviour {
    /// Handle an event
    fn handle_event(&self, event: Event) -> Event;
}
