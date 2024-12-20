const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let w = roget::Wordle::new();
    for answer in GAMES.split_whitespace() {
        println!("answer: `{}`", answer);
        let gusser = roget::algorithms::Naive::new();
        if let Some(score) = w.play(answer, gusser) {
            println!("guessed `{}` in {}", answer, score);
        } else {
            eprintln!("failed to guess");
        }
    }
}
