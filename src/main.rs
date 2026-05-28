const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let guesser = worust::algos::amateur::new();
    for ans in GAMES.split_whitespace() {
        worust::play(ans, guesser);
    }
    println!("Hello, world!");
}
