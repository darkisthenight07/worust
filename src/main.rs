const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for ans in GAMES.split_whitespace() {
        let guesser: worust::algos::Starter = worust::algos::Starter::new();
        worust::play(ans, guesser);
    }
    println!("Hello, world!");
}
