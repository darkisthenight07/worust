pub mod algos;

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> usize {
    let mut prev: Vec<Guess> = Vec::new();

    for i in 0.. {
        let guess: String = guesser.guess(&prev[..]);
        if guess == answer {
            return i;
        }

        let color: [Color; 5] = Color::check(answer, &guess);
        prev.push(Guess {
            word: guess,
            color,
        });
    }
    panic!("didn't finish!? (that's what she said)");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Green,
    Yellow,
    Gray,
}

impl Color {
    fn check(answer: &str, guess: &str) -> [Self; 5] {
        assert!(answer.len() == 5 && guess.len() == 5);

        let answer_bytes = answer.as_bytes();
        let guess_bytes = guess.as_bytes();

        let mut result: [Color; 5] = [Color::Gray; 5];
        let mut used: [bool; 5] = [false; 5];

        for i in 0..5 {
            if answer_bytes[i] == guess_bytes[i] {
                result[i] = Color::Green;
                used[i] = true;
            }
        }
        for i in 0..5 {
            if result[i] == Color::Green {
                continue;
            }
            for j in 0..5 {
                if !used[j] && answer_bytes[j] == guess_bytes[i] {
                    result[i] = Color::Yellow;
                    used[j] = true;
                    break;
                }
            }
        }

        result
    }
}

pub struct Guess {
    pub word: String,
    pub color: [Color; 5],
}

pub trait Guesser {
    fn guess(&mut self, prev: &[Guess]) -> String;
}