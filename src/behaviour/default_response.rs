use event::Event;
use behaviour::Behaviour;

pub struct DefaultResponse {
    response: String,
}

impl DefaultResponse {
    pub fn new(response: &str) -> DefaultResponse {
        DefaultResponse {
            response: response.to_string()
        }
    }
}

impl Behaviour for DefaultResponse {
    fn react(&self, to: Event) -> Event {
        match to {
            _ => Event::Tell(self.response.clone())
        }
    }
}
