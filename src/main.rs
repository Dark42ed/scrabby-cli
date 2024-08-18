use scrabby::board;

mod game;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

pub const BOARD_SIZE: usize = 21;

lazy_static::lazy_static! {
    static ref WORDS: String = std::fs::read_to_string("words.txt").unwrap();
    static ref WORD_LIST: &'static [&'static str] = Box::leak(WORDS.lines().collect::<Vec<_>>().into_boxed_slice());
}

fn main() -> Res<()> {
    let mut board = board::Board::new(BOARD_SIZE);

    loop {
        println!();
        game::play(&mut board).unwrap();
    }
}
