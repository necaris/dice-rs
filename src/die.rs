use rand::Rng;
use rand::prelude::thread_rng;

#[derive(Debug, PartialEq, Clone)]
pub struct Die {
    pub sides: u8,
}

impl Die {
    pub fn roll(&self) -> u8 {
        let mut rng = thread_rng();
        rng.gen_range(1, self.sides + 1)
    }
}

#[derive(Debug, PartialEq)]
pub struct DiceSpec<'lifetime> {
    pub dice: Vec<Die>,
    pub compare_to: Option<&'lifetime str>,
}

impl DiceSpec<'_> {

    // TODO: compare `self.compare_to` against fields in a profile
    // map, somehow
    pub fn roll(&self) -> Vec<u8> {
        self.dice.iter().map(|d| d.roll()).collect::<Vec<u8>>()
    }
}
