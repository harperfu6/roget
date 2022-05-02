const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let gusser = roget::algorithms::Naive::new();
        roget::play(answer, gusser);
    }
}
