mod board;
mod letter;
mod game;
mod computer;

type Res<T> = Result<T, Box<dyn std::error::Error>>;


lazy_static::lazy_static! {
    static ref WORDS: String = std::fs::read_to_string("words.txt").unwrap();
    static ref WORD_LIST: &'static [&'static str] = Box::leak(WORDS.split('\n').collect::<Vec<_>>().into_boxed_slice());
}

fn main() -> anyhow::Result<()> {
    let mut board = board::Board::new();

    loop {
        println!();
        game::play(&mut board).unwrap();
    }
}
