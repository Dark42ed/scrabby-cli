use scrabby::{Board, Direction, Letter};

use crate::Res;
use std::io::{stdin, stdout, Write};

fn input(prompt: &str) -> Res<String> {
    let mut output = String::with_capacity(5);
    print!("{}", prompt);
    stdout().flush()?;
    stdin().read_line(&mut output)?;
    Ok(output)
}

fn menu(name: &str, options: &[&str]) -> Res<u32> {
    let mut num = 0u32;

    while !(1..=options.len()).contains(&(num as _)) {
        println!("{}", name);
        println!("{}", "-".repeat(name.len()));
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }
        if let Ok(n) = input(">")?.trim().parse::<u32>() {
            num = n as _;
        }
    }

    Ok(num)
}

pub fn play(board: &mut Board) -> Res<()> {
    match menu(
        "Turn",
        &[
            "Your turn",
            "Other person's turn",
            "Print Board",
            "Load Board",
            "Save Board",
        ],
    )? {
        1 => calculate_best_move(board),
        2 => update_board_from_input(board),
        3 => {
            board.print();
            Ok(())
        }
        4 => {
            *board = load_board()?;
            Ok(())
        }
        5 => save_board(board),
        _ => unreachable!(),
    }
}

fn load_board() -> Res<Board> {
    let name = input("Board name >")?;
    Ok(bincode::deserialize(&std::fs::read(name.trim())?)?)
}

fn save_board(board: &mut Board) -> Res<()> {
    let mut i = 1;
    loop {
        let name = "board".to_string() + &i.to_string() + ".bin";
        if std::path::Path::new(&name).exists() {
            i += 1;
        } else {
            std::fs::write(&name, bincode::serialize(board)?)?;
            println!("Saved to {}", &name);
            break;
        }
    }
    Ok(())
}

fn calculate_best_move(board: &mut Board) -> Res<()> {
    let letters = input("Input your letters >")?
        .trim_end_matches(|x| x == '\n' || x == '\r')
        .to_uppercase()
        .chars()
        .map(|ch| Letter::from_char(ch))
        .collect::<Vec<_>>();

    let best = scrabby::computer::best_moves(board, &letters, &crate::WORD_LIST);

    let mut buf = String::new();
    for b in best {
        buf.clear();
        println!(
            "Word: {}\tScore: {}\tLocation: {:?}",
            b.word,
            b.get_score(board),
            b.location
        );
        let letters_to_print = b.word.as_bytes().iter().enumerate().map(|(i, c)| {
            let letter_location = match b.direction {
                Direction::Down => b.location + (i * board.size()),
                Direction::Right => b.location + i,
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

fn update_board_from_input(board: &mut Board) -> Res<()> {
    let word = input("Enter the word >")?.trim().to_uppercase();
    let location: Vec<i32> = input("Enter the location X,Y >")?
        .trim()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect();
    let direction = match menu("Enter the direction >", &["Right", "Down"])? {
        1 => Direction::Right,
        2 => Direction::Down,
        _ => unreachable!(),
    };

    let location = (location[0] - 1, location[1] - 1);

    let static_word = crate::WORD_LIST.iter().find(|&&x| x == &word);
    if let Some(w) = static_word {
        board.make_move(location.1 as _, location.0 as _, w, direction);
    } else {
        let s = Box::leak(word.into_boxed_str());
        board.make_move(location.1 as _, location.0 as _, s, direction);
    }

    board.print();

    Ok(())
}
