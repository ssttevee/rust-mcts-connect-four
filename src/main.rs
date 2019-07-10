extern crate termion;
extern crate rand;

pub mod game;

use rand::Rng;
use std::error;
use std::io;
use std::fmt;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct Done;

impl fmt::Display for Done {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "done")
    }
}

impl error::Error for Done {}

fn next_move(game: &mut game::Game, input: &mut io::Lines<io::StdinLock<'_>>) -> Option<Box<dyn error::Error>> {
    let line = match input.next().transpose() {
        Err(err) => return Some(Box::<dyn error::Error>::from(err)),
        Ok(line) => match line {
            None => return Some(Box::new(Done)),
            Some(line) => line,
        },
    };
    
    let col = match usize::from_str(line.as_ref()) {
        Err(err) => return Some(Box::<dyn error::Error>::from(err)),
        Ok(col) => col,
    };

    game.drop(col).and_then(|err| Some(Box::<dyn error::Error>::from(err)))
}

fn main() {
    let mut rng = rand::thread_rng();

    use game::Player::{Player1, Player2};

    let mut game = game::new();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while !game.over() {
        if let Some(ref err) = match game.current_player() {
            Player1 => {
                println!("{}{}", termion::clear::All, game);
                print!("What's your move?: ");
                io::stdout().flush().unwrap();

                next_move(&mut game, &mut lines)
            },
            Player2 => {
                let col: usize = rng.gen_range(0, game.cols()) + 1;
                game.drop(col)
            }
        } {
            println!("{}", err);
            return
        }
    }

    println!("{}{}", termion::clear::All, game);
    println!("{} wins", game.winner().unwrap());
}
