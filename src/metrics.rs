use crate::superglide::Percentage;

#[derive(Debug)]
pub struct Metrics {
    attempts: u32,
    potential_superglides: u32,
    crouch_first: u32,
    crouch_late: u32,
    overall: f64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            attempts: 0,
            potential_superglides: 0,
            crouch_first: 0,
            crouch_late: 0,
            overall: 1.0,
        }
    }

    pub fn record_possible_superglide(&mut self, superglide_chance: &Percentage) {
        let attempts = f64::from(self.attempts);
        self.overall = (superglide_chance.0 + (attempts * self.overall)) / (attempts + 1.0);
        self.attempts += 1;
        self.potential_superglides += 1;
    }
    pub fn record_crouch_first(&mut self) {
        let attempts = f64::from(self.attempts);
        self.overall = (attempts * self.overall) / (attempts + 1.0);
        self.attempts += 1;
        self.crouch_first += 1;
    }
    pub fn record_crouch_late(&mut self) {
        let attempts = f64::from(self.attempts);
        self.overall = (attempts * self.overall) / (attempts + 1.0);
        self.attempts += 1;
        self.crouch_late += 1;
    }
}
