pub mod algos;

pub fn play(answer: &'static str, guesser: G) -> usize{
    //play the rounds
    let mut prev = Vec::new();
    for i in 0.. {
        let guess = guesser.guess(&prev[..]);
        if guess == answer {
            return i;
        }
        let match = Match::check(answer, guess);
        prev.push(Guess { word: guess, match: match });
    }
    panic!("didn't finish!? (that's what she said)");
}

pub enum Match {
    Green,
    Yellow,
    Gray,
}
impl Match {
    fn check(answer: &str, guess: &str) -> [Self; 5] {
        assert!(answer.len() == 5 && guess.len() == 5); 
        todo!()
    }
}

pub struct Guess {
    pub word: String,
    pub match: [Match; 5],
}

pub trait Guesser {
    fn guess(&mut self, prev:&[Guess]) -> String;
}