use world::World;
use world::campaign::Campaign;

/// The engine to run the game
pub struct Engine<W: World> {
    campaigns: Vec<Campaign<W>>,
}

impl<W: World> Engine<W> {
    /// start the engine and run the game with th current configuration
    pub fn run(&mut self) {
        unimplemented!()
    }

    /// Add a campaign to the engine
    pub fn add_campaign(&mut self, campaign: Campaign<W>) {
        self.campaigns.push(campaign);
    }
}
