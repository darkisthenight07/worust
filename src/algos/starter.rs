use crate::{Guesser, Guess};

pub struct Starter;

impl Starter {
    pub fn new() -> Self {
        Starter
    }
}

impl Guesser for Starter {
    fn guess(&mut self, _prev: &[Guess]) -> String {
        todo!()
    }
}