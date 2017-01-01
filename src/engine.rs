use world::World;
use world::campaign::Campaign;

/// The engine to run the game
pub struct Engine<W: World> {
    campaigns: Vec<Campaign<W>>,
    campaign_counter: usize,
}

impl<W: World> Engine<W> {
    /// Creates a new instance of `Engine`
    pub fn new() -> Engine<W> {
        Engine {
            campaigns: Vec::new(),
            campaign_counter: 0,
        }
    }

    /// start the engine and run the game with th current configuration
    pub fn run(&mut self) {
        unimplemented!()
    }

    /// Returns the currently selected campaign
    pub fn current_campaign(&self) -> &Campaign<W> {
        &self.campaigns[self.campaign_counter]
    }

    /// Add a campaign to the engine
    pub fn add_campaign(&mut self, campaign: Campaign<W>) {
        self.campaigns.push(campaign);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use world::two_dimensional::World2d;

    #[test]
    fn new_engine() {
        let engine: Engine<World2d> = Engine::new();
    }
}
