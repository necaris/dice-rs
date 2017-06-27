/// Provides the basic `Die` type which is "rolled"
use std::fmt;
use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng, ThreadRng};

pub struct Die {
    sides: u32,
    rng: ThreadRng,
    range: Range<u32>,
}

impl Die {
    pub fn new(sides: u32) -> Die {
        Die {
            sides: sides,
            rng: thread_rng(),
            // Since Range provides an interval half open on the right
            // (i.e. does not include its endpoint) we need to do this
            range: Range::new(1, sides + 1)
        }
    }

    pub fn roll(&mut self) -> u32 {
        self.range.ind_sample(&mut self.rng)
    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "d{}", self.sides)
    }
}
