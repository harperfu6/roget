const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let w = roget::Wordle::new();
    for answer in GAMES.split_whitespace() {
        let gusser = roget::algorithms::Naive::new();
        w.play(answer, gusser);
    }
}
