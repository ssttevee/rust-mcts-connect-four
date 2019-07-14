#[macro_use]
extern crate itertools;

extern crate termion;
extern crate rand;

pub mod game;
pub mod mcts;

use mcts::MCTS;

use std::error;
use std::io;
use std::fmt;
use std::io::prelude::*;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug)]
struct Done;

impl fmt::Display for Done {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "done")
    }
}

impl error::Error for Done {}

fn player_move(game: &mut game::Game, input: &mut io::Lines<io::StdinLock<'_>>) -> Result<(), Done> {
    let mut message: Option<String> = None;

    loop {
        println!("{}", game);
        // println!("{}{}", termion::clear::All, game);
        if let Some(msg) = message {
            println!("{}", msg);
        }
        print!(
            "What's your move? [{}]: ",
            game.valid_moves().iter()
                .map(|col| col.to_string())
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
        
        let col = match usize::from_str(line.as_ref()) {
            Err(err) => {
                message = Some(format!("{}", err));
                continue;
            },
            Ok(col) => col,
        };

        match game.drop(col) {
            Err(err) => {
                message = Some(format!("{}", err));
            },
            _ => break Ok(()),
        }
    }
}

fn mcts_move(game: &mut game::Game, mcts: &MCTS) {
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

    game.drop(col).unwrap();
}

fn main() {
    use game::Player::{Player1, Player2};

    let mut rng = rand::thread_rng();
    let mut mcts = mcts::new();
    let mut game = game::new();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while !game.over() {
        match game.current_player() {
            Player1 => {
                if let Err(err) = player_move(&mut game, &mut lines) {
                    println!("{}", err);
                    return
                }
            },
            Player2 => {
                let results = mcts.think(&mut rng, &game, Duration::new(1, 0));
                println!("ran {} simulations; ({},{},{})", results[3], results[0], results[1], results[2]);
                mcts_move(&mut game, &mcts);
            }
        }
    }

    println!("{}", game);
    // println!("{}{}", termion::clear::All, game);
    match game.winner() {
        Some(player) => match player {
            Player1 => println!("You win!"),
            Player2 => println!("You lose!"),
        },
        None => println!("It's a tie!"),
    }
}
