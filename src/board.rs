use crate::letter::Letter;
use colored::Colorize;
use std::borrow::Cow;
use serde_derive::{
    Serialize,
    Deserialize
};

pub const BOARD_SIZE: usize = 21;
pub const ARRAY_SIZE: usize = BOARD_SIZE * BOARD_SIZE;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    inner: Vec<Option<Letter>>, // ARRAY_SIZE
    moves: Vec<Move>
}

impl Board {
    pub fn new() -> Board {
        Board {
            inner: vec![None; ARRAY_SIZE],
            moves: Vec::new()
        }
    }

    pub fn iter_letters(&self) -> impl Iterator<Item = Letter> + '_ {
        self.inner.iter().filter_map(|x| *x)
    }

    pub fn enumerate_letters(&self) -> impl Iterator<Item = (usize, Letter)> + '_ {
        self.inner.iter().enumerate().filter_map(|x| x.1.map(|y| (x.0, y)))
    }

    pub fn make_move(&mut self, mut row: u8, mut column: u8, word: &'static str, direction: Direction) {
        for char in word.chars() {
            self.set(row as _, column as _, Some(Letter::from_char(char)));
            match direction {
                Direction::Down => row += 1,
                Direction::Right => column += 1
            };
        }

        self.moves.push(Move::new(
            convert_to_index(row, column),
            direction,
            word,
        ));
    }

    pub fn get(&self, row: u8, column: u8) -> Option<Letter> {
        self.inner[convert_to_index(row, column)]
    }

    pub fn set(&mut self, row: u8, column: u8, letter: Option<Letter>) {
        self.inner[convert_to_index(row, column)] = letter;
    }

    pub fn get_index(&self, index: usize) -> Option<Letter> {
        self.inner.get(index).and_then(|x| *x)
    }

    pub fn set_index(&mut self, index: usize, letter: Option<Letter>) {
        self.inner[index] = letter;
    }

    pub fn print(&self) {
        for i in 0..BOARD_SIZE {
            for l in self.inner[i*BOARD_SIZE..(i+1)*BOARD_SIZE].iter().map(|x| match x {
                Some(y) => y.to_char(),
                None => '.'    
            }) {
                print!("{} ", l);
            }
            println!();
        }
    }

    pub fn print_highlight(&self, highlight: &[(Letter, usize)]) {
        for (i, l) in self.inner.iter().enumerate() {
            if i % BOARD_SIZE == 0 {
                println!();
            }
            if let Some(x) = highlight.iter().find(|f| f.1 == i) {
                print!("{} ", x.0.to_char().to_string().red())
            } else {
                if let Some(letter) = l {
                    print!("{} ", letter.to_char());
                } else {
                    print!(". ",);
                }
            }
        }
        println!();
    }
}

pub fn convert_to_index(row: u8, column: u8) -> usize {
    row as usize * BOARD_SIZE + column as usize
}

pub fn convert_from_index(index: usize) -> (u8, u8) {
    ((index % BOARD_SIZE) as u8, (index / BOARD_SIZE) as u8)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub location: usize,
    pub direction: Direction,
    pub word: Cow<'static, str>
}

impl Move {
    pub fn new(location: usize, direction: Direction, word: &'static str) -> Move {
        Move {
            location,
            direction,
            word: Cow::Borrowed(word)
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Right,
    Down
}

