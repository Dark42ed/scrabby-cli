use std::io::{
    Write,
    stdin,
    stdout
};
use crate::board::Board;
use crate::board::Direction;
use crate::computer;
use crate::Res;

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
    match menu("Turn", &["Your turn", "Other person's turn", "Print Board", "Load Board", "Save Board"])? {
        1 => calculate_best_move(board),
        2 => update_board_from_input(board),
        3 => {
            board.print();
            Ok(())
        }
        4 => {
            *board = load_board()?;
            Ok(())
        },
        5 => save_board(board),
        _ => unreachable!()
    }
}

fn load_board() -> Res<Board> {
    let name = input("Board name >")?;
    return Ok(bincode::deserialize(&std::fs::read(name.trim())?)?)
}

fn save_board(board: &mut Board) -> Res<()> {
    let mut i=1;
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
    return Ok(())
}

fn calculate_best_move(board: &mut Board) -> Res<()> {
    let letters = input("Input your letters >")?.trim_end_matches(|x| x == '\n' || x == '\r').to_uppercase();

    computer::run(board, letters.as_bytes())?;

    Ok(())
}

fn update_board_from_input(board: &mut Board) -> Res<()> {
    let word = input("Enter the word >")?.trim().to_uppercase();
    let location: Vec<i32> = input("Enter the location X,Y >")?.trim().split(",").filter_map(|x| x.parse().ok()).collect();
    let direction = match menu("Enter the direction >", &["Right", "Down"])? {
        1 => Direction::Right,
        2 => Direction::Down,
        _ => unreachable!()
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