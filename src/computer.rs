use crate::board::Board;
use crate::board::Direction;
use crate::board;
use crate::board::Move;
use crate::letter::Letter;
use crate::letter;
use std::io::Write;

use crate::Res;

pub fn run(board: &mut Board, letters: &[u8]) -> Res<()> {
    let mut rack = Vec::from(letters);

    let mut best: Vec<(usize, Move)> = Vec::new();
    for (location, letter) in board.enumerate_letters() {
        rack.push(letter.to_char() as u8);

        let words = get_createable_words(&rack);

        for word in words {
            let move_verification = verify_move(board, location, word);
            best.extend(move_verification.iter().map(|x| (get_word_score(x.word.as_ref(), x.location, x.direction), (*x).clone())));
        }

        rack.pop();
    }

    //let highlight = best[0..10].iter().map(|x| x.1).collect::<Vec<_>>();
    best.sort_unstable_by_key(|x| x.0);
    best.reverse();

    let mut buf = String::new();
    for b in best {
        buf.clear();
        println!("Word: {}\tScore: {}\tLocation: {:?}", b.1.word, b.0, b.1.location);
        let letters_to_print = b.1.word.as_bytes().iter().enumerate().map(|(i, c)| {
            let letter_location = match b.1.direction {
                Direction::Down => b.1.location + (i * board::BOARD_SIZE),
                Direction::Right => b.1.location + i
            };

            (Letter::from_char(*c as _), letter_location)
        });
        board.print_highlight(&letters_to_print.collect::<Vec<_>>());

        print!("Enter \"done\" to finish >");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut buf)?;
        buf.make_ascii_lowercase();

        if buf.trim() == "done" {
            break;
        }
    }


    Ok(())
}

pub fn get_createable_words(rack: &[u8]) -> impl Iterator<Item = &&'static str> + '_ {
    crate::WORD_LIST.iter().filter(|word| can_create_word(rack, word))
}

pub fn can_create_word(rack: &[u8], word: &str) -> bool {
    let mut rack = Vec::from(rack);
    let mut blank_count = rack.iter().filter(|&&x| x == b' ').count();

    'outer: for ch in word.as_bytes() {
        for (i, letter) in rack.iter().enumerate() {
            if letter == ch {
                rack[i] = b' ';
                continue 'outer;
            }
        }
        if blank_count > 0 {
            blank_count -= 1;
            continue 'outer;
        }
        return false;
    }
    true
}

pub fn get_word_score(word: &str, mut location: usize, direction: Direction) -> usize {
    let location_change = match direction {
        Direction::Right => 1,
        Direction::Down => board::BOARD_SIZE
    };
    let mut sum = 0;
    let mut word_mul = 1;
    for l in word.as_bytes() {
        sum += Letter::from_char(*l as char).score() as usize * letter::LETTER_MULT[location] as usize;
        if let Some(mul) = letter::WORD_MULT.get(location) {
            word_mul *= mul;
        }

        location += location_change;
    }
    sum *= word_mul as usize;

    sum
}

// Not using Move struct because direction is to be determined.
pub fn verify_move(board: &Board, location: usize, word: &'static str) -> Vec<Move> {
    let mut good_ones = Vec::new();

    for direction in [Direction::Down, Direction::Right] {
        'outer: for letter in word.as_bytes().iter().enumerate().filter(|x| Letter::from_char(*x.1 as char) == board.get_index(location).unwrap()) {
            let string_position = letter.0;
            let starting_position = match direction {
                Direction::Down => location - (string_position * board::BOARD_SIZE),
                Direction::Right => location - string_position
            };
            let (starting_column, starting_row) = crate::board::convert_from_index(starting_position);

            for (i, word_letter) in word.as_bytes().iter().enumerate() {
                let test_position = match direction {
                    Direction::Down => starting_position.wrapping_add(i * board::BOARD_SIZE),
                    Direction::Right => starting_position.wrapping_add(i)
                };

                let (current_column, current_row) = crate::board::convert_from_index(test_position);
                if match direction {
                    Direction::Down => current_column != starting_column,
                    Direction::Right => current_row != starting_row
                } {
                    continue 'outer;
                }

                if test_position >= board::ARRAY_SIZE {
                    continue 'outer;
                }
                
                let test = board.get_index(test_position);
                if let Some(test_inner) = test {
                    if test_inner != Letter::from_char(*word_letter as char) {
                        continue 'outer;
                    }
                }
            }

            good_ones.push(Move::new(
                starting_position,
                direction,
                word
            ));
        }
    }

    good_ones
}