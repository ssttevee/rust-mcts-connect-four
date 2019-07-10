extern crate termion;

pub mod board;

use std::error;
use std::fmt;
use std::cmp;
use std::iter;

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Player::{Player1, Player2};

        write!(f, "Player {}", match self {
            Player1 => 1,
            Player2 => 2,
        })
    }
}

impl board::Token for Player {
    fn color(&self) -> &dyn termion::color::Color {
        use self::Player::{Player1, Player2};

        match self {
            Player1 => &termion::color::Red,
            Player2 => &termion::color::Green,
        }
    }
}

#[derive(Debug)]
pub struct GameOverError;

impl fmt::Display for GameOverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "game is already over")
    }
}

impl error::Error for GameOverError {}

pub struct Game {
    board: board::Board<Player>,
    win_len: usize,

    current_player: Player,

    winner: Option<Player>,
}

impl Game {
    fn search_ranges<'a>(&'a self, col: usize, row: usize) -> Box<dyn iter::Iterator<Item = Box<dyn iter::Iterator<Item = (usize, usize)> + 'a>> + 'a> {
        let mut iter: Box<dyn iter::Iterator<Item = Box<dyn iter::Iterator<Item = (usize, usize)> + 'a>> + 'a> = Box::new(iter::empty());

        // check vertical win condition
        if row > self.win_len - 2 {
            iter = Box::new(iter.chain(
                iter::once(
                    Box::new(iter::repeat(col).zip(row - (self.win_len - 1)..row + 1)) as Box<dyn iter::Iterator<Item = (usize, usize)> + 'a>
                )
            ));
        }

        let max_search_col = 1 + self.cols() - self.win_len;
        let min_col = cmp::max(0, col as i8 - (self.win_len - 1) as i8) as usize;
        let max_col = cmp::min(max_search_col, col + 1);

        // check horizontal win conditions
        iter = Box::new(iter.chain(
            (min_col..max_col).map(
                move |col| Box::new((col..col + self.win_len).zip(iter::repeat(row))) as Box<dyn iter::Iterator<Item = (usize, usize)> + 'a>
            )
        ));


        // check diagonal win conditions (top-left to bottom-right)
        let bot_max_search_row = 1 + self.rows() - self.win_len;
        if row < col + bot_max_search_row && col < row + max_search_col {
            let min_row = cmp::max(0, row as i8 - (self.win_len - 1) as i8) as usize;
            let max_row = cmp::min(bot_max_search_row, row + 1);

            let start_col = cmp::max(0, col as i8 - (row - min_row) as i8) as usize;
            let start_row = cmp::max(0, row as i8 - (col - min_col) as i8) as usize;

            let end_col = cmp::min(max_search_col, col + max_row - row);
            let end_row = cmp::min(bot_max_search_row, row + max_col - col);

            // println!(
            //     "\t({},{})..({},{})",
            //     start_col,
            //     start_row,
            //     end_col,
            //     end_row,
            // );

            iter = Box::new(iter.chain(
                (start_col..end_col).zip(start_row..end_row).map(
                    move |(col, row)| Box::new((col..col + self.win_len).zip(row..row + self.win_len)) as Box<dyn iter::Iterator<Item = (usize, usize)> + 'a>
                )
            ))
        };

        let top_min_search_row = self.win_len - 1;
        if row + col > self.win_len - 2 && col + row < self.cols() + self.rows() - self.win_len {
            let min_row = cmp::max(top_min_search_row as i8, row as i8 - 1) as usize;
            let max_row = cmp::min(self.rows() - 1, row + self.win_len - 1);

            let start_col = cmp::max(0, (col + row) as i8 - max_row as i8) as usize;
            let start_row = cmp::min(self.rows() - 1, (col + row) - min_col);

            let end_col = cmp::min(max_search_col, 1 + col - (min_row as i8 - row as i8).max(0) as usize);
            let end_row = cmp::max(top_min_search_row as i8 - 1, row as i8 - 1 - (max_col as i8 - col as i8 - 1).min(0)) as usize;
            // println!(
            //     "\t({},{})..({},{})",
            //     start_col,
            //     start_row,

            //     end_col,
            //     end_row,
            // );

            iter = Box::new(iter.chain(
                (start_col..end_col).zip((end_row + 1..start_row + 1).rev()).map(
                    move |(col, row)| Box::new((col..col + self.win_len).zip((row + 1 - self.win_len .. row + 1).rev())) as Box<dyn iter::Iterator<Item = (usize, usize)> + 'a>
                )
            ))
        }

        iter
    }

    fn check_winner(&self, player: Player, col: usize, row: usize) -> Option<Box<[(usize, usize)]>> {
        let ranges = self.search_ranges(col, row);
        'outer: for range in ranges {
            let v = range.collect::<Vec<_>>();
            for i in 0 .. v.len() {
                if !self.board[v[i]].has(player) {
                    continue 'outer;
                }
            }

            return Some(v.into_boxed_slice())
        }

        None
    }

    pub fn drop(&mut self, col: usize) -> Option<Box<dyn error::Error>> {
        if self.over() {
            return Some(Box::new(GameOverError))
        }

        let row = match self.board.drop(col, self.current_player) {
            Err(err) => return Some(Box::<dyn error::Error>::from(err)),
            Ok(row) => row,
        };

        if let Some(cells) = self.check_winner(self.current_player, col-1, row) {
            self.winner = Some(self.current_player);
            for (col, row) in cells.into_iter() {
                self.board.highlight(*col, *row);
            }
        }

        self.current_player = match self.current_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };

        None
    }

    pub fn over(&self) -> bool {
        self.winner != None
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn cols(&self) -> usize {
        self.board.cols()
    }

    pub fn rows(&self) -> usize {
        self.board.rows()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}

pub fn custom(cols: usize, rows: usize, win_length: usize) -> Game {
    Game {
        board: board::new(cols, rows),
        win_len: win_length,

        current_player: Player::Player1,

        winner: None,
    }
}

pub fn new() -> Game {
    custom(7, 6, 4)
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use self::rand::Rng;

    fn print_search_ranges(cols: usize, rows: usize, wins: usize, col: usize, row: usize) {
        let mut game = custom(cols, rows, wins);

        game.board.highlight(col, row);
        for range in game.search_ranges(col, row) {
            let mut board = game.board.clone();
            for (col, row) in range {
                print!("({},{}) ", col, row);
                board.fill(col, row, Player::Player1);
            }

            println!();

            println!("{}", board)
        }
    }

    #[test]
    fn test_game_search_ranges() {
        let mut rng = rand::thread_rng();

        let cols: usize = rng.gen_range(3, 10);
        let rows: usize = rng.gen_range(3, 10);
        let wins: usize = rng.gen_range(2, cmp::min(cols, rows) + 1);

        print_search_ranges(cols, rows, wins, rng.gen_range(0, cols), rng.gen_range(0, rows))

        // let game = new();

        // game.search_ranges(6, 1);
        // game.search_ranges(6, 2);
        // game.search_ranges(5, 2);
        // game.search_ranges(5, 3);
        // game.search_ranges(4, 3);
        // game.search_ranges(4, 4);
        
        // game.search_ranges(0, 5);
        // game.search_ranges(1, 5);
        // game.search_ranges(0, 4);
        // game.search_ranges(1, 4);
        // game.search_ranges(5, 1);

        // game.search_ranges(2, 3);
        // game.search_ranges(3, 4);
        // game.search_ranges(2, 4);
        // game.search_ranges(2, 2);
        // game.search_ranges(2, 1);

        // game.search_ranges(3, 2);
        // game.search_ranges(4, 2);
        // game.search_ranges(5, 1);
        // game.search_ranges(4, 1);
        // game.search_ranges(3, 1);

        // let mut board = game.board.clone();
        // for col in 0..game.board.cols() {
        //     for row in 0..game.board.rows() {
        //         if row + col > game.win_len - 2 && col + row < game.board.cols() + game.board.rows() - game.win_len {
        //             board.fill(col, row, Player::Player1)
        //         } else {
        //             board.highlight(col, row)
        //         }
        //     }
        // }

        // println!("{}", board)
    }
}