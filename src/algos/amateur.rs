use crate::{Guesser, Guess, Match};

pub struct Amateur;

impl Amateur {
    pub fn new() -> Self {
        Self
    }
}

impl Guesser for Amateur {
    fn guess(&mut self, prev: &[Guess]) -> String {
        todo!()
    }
}