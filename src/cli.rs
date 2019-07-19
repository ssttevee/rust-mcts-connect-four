use std::error;
use std::io;
use std::fmt;
use std::io::prelude::*;
use std::str::FromStr;
use std::time::Duration;

use game::Game;
use mcts::MCTS;

#[derive(Debug)]
struct Done;

impl fmt::Display for Done {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "done")
    }
}

impl error::Error for Done {}

fn player_move<F: Fn(&Game) -> ()>(game: &mut Game, input: &mut io::Lines<io::StdinLock<'_>>, print_board: F) -> Result<(usize, usize), Done> {
    let mut message: Option<String> = None;

    loop {
        if let Some(msg) = message {
            println!("{}", termion::clear::All);
            print_board(game);
            println!("{}", msg);
        } else {
            print_board(game);
        }

        print!(
            "What's your move? [{}]: ",
            game.valid_moves().iter()
                .map(|col| (col + 1).to_string())
                .collect::<Vec<_>>()
                .join(","),
        );

        io::stdout().flush().unwrap();
        
        let line = match input.next().transpose() {
            Err(err) => {
                message = Some(format!("{}", err));
                continue;
            },
            Ok(line) => match line {
                None => return Err(Done),
                Some(line) => line,
            },
        };

        if line.len() == 0 {
            message = Some(String::from("please enter a number"));
            continue;
        }
        
        let mut col = match usize::from_str(line.as_ref()) {
            Err(err) => {
                message = Some(format!("{}", err));
                continue;
            },
            Ok(col) => col,
        };

        if col < 1 || col > game.cols() {
            message = Some(format!("please enter a valid move"));
            continue;
        }

        col -= 1;
        match game.drop(col) {
            Err(err) => {
                message = Some(format!("{}", err));
            },
            Ok(row) => break Ok((col, row)),
        }
    }
}

fn mcts_move(game: &mut Game, mcts: &MCTS) -> (usize, usize) {
    let state = game.state();
    let valid_moves = game.valid_moves();
    let winrates = mcts.move_weights(&state, &valid_moves);

    // print out move win rates
    println!(
        "{}",
        winrates.iter()
            .enumerate()
            .map(|(i, winrate)| format!("{}: {}", valid_moves[i], winrate))
            .collect::<Vec<_>>()
            .join("\t"),
    );
    
    let (col, _) = valid_moves.iter()
        .zip(winrates)
        .fold((0, -1.0), |(prev_col, prev_weight), (col, weight)| if prev_weight < weight {
            (*col, weight)
        } else {
            (prev_col, prev_weight)
        });

    (col, game.drop(col).unwrap())
}

fn print_board_top(cols: usize) {
    println!("| {} |", (1..=cols).map(|i| i.to_string()).collect::<Vec<_>>().join(" | "))
}

fn print_board(game: &Game, last_move: Option<(usize, usize)>) {
    let mut board = game.board();
    if let Some((col, row)) = last_move {
        board.highlight(col, row);
    }

    print_board_top(game.cols());
    println!("{}", board);
}

pub fn start() {
    use board::Token::{Player1, Player2};

    let mut mcts = MCTS::new();
    let mut game = Game::new();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut last_move: Option<(usize, usize)> = None;
    while !game.over() {
        match game.current_player() {
            Player1 => match player_move(&mut game, &mut lines, |game: &Game| print_board(game, last_move)) {
                Err(err) => {
                    println!("{}", err);
                    return
                },
                Ok(coord) => {
                    last_move = Some(coord);
                },
            },
            Player2 => {
                println!("{}", termion::clear::All);
                print_board(&game, last_move);
                print!("thinking...");
                io::stdout().flush().unwrap();

                let results = mcts.think(&game, Duration::new(1, 0));

                println!("{}", termion::clear::All);
                println!("ran {} simulations; ({},{},{})", results[3], results[0], results[1], results[2]);
                last_move = Some(mcts_move(&mut game, &mcts));
            }
        }
    }

    let mut board = game.board();
    let winner = match game.winner() {
        Some((player, cells)) => {
            for (col, row) in cells.into_iter() {
                board.highlight(*col, *row);
            };

            Some(player)
        },
        None => None,
    };

    println!("{}", termion::clear::All);
    print_board_top(board.cols());
    println!("{}", board);

    match winner {
        Some(player) => match player {
            Player1 => println!("You win!"),
            Player2 => println!("You lose!"),
        },
        None => println!("It's a tie!"),
    }
}