#[cfg(not(target_arch = "wasm32"))]
extern crate termion;

use std::error;
use std::fmt;
use std::cmp;
use std::iter;

use board::{Board, Token};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct GameOverError;

impl fmt::Display for GameOverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "game is already over")
    }
}

impl error::Error for GameOverError {}

pub type State = Vec<u8>;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone)]
pub struct Game {
    board: Board,
    win_len: usize,

    current_player: Token,

    winner: Option<(Token, Box<[(usize, usize)]>)>,
}

impl Game {
    pub fn custom(cols: usize, rows: usize, win_length: usize) -> Game {
        Game {
            board: Board::new(cols, rows),
            win_len: win_length,

            current_player: Token::Player1,

            winner: None,
        }
    }

    pub fn new() -> Game {
        Game::custom(7, 6, 4)
    }

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

    fn check_winner(&self, player: Token, col: usize, row: usize) -> Option<Box<[(usize, usize)]>> {
        let ranges = self.search_ranges(col, row);
        'outer: for range in ranges {
            let v = range.collect::<Vec<_>>();
            for (col, row) in v.clone() {
                if !self.board.token_at(col, row)
                    .and_then(|p| Some(p == player))
                    .unwrap_or(false) {
                    continue 'outer;
                }
            }

            return Some(v.into_boxed_slice())
        }

        None
    }

    pub fn drop(&mut self, col: usize) -> Result<usize, Box<dyn error::Error>> {
        use board::Token::{Player1, Player2};

        if self.over() {
            return Err(Box::new(GameOverError))
        }

        let row = self.board.drop(col, self.current_player)?;

        if let Some(cells) = self.check_winner(self.current_player, col, row) {
            self.winner = Some((self.current_player, cells));
        }

        self.current_player = match self.current_player {
            Player1 => Player2,
            Player2 => Player1,
        };

        Ok(row)
    }

    pub fn valid_moves(&self) -> Vec<usize> {
        (0..self.cols()).filter_map(|col| match self.board.token_at(col, self.rows()-1) {
            None => Some(col),
            Some(_) => None,
        }).collect()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Game {
    pub fn over(&self) -> bool {
        self.winner != None || self.valid_moves().len() == 0
    }

    pub fn current_player(&self) -> Token {
        self.current_player
    }

    pub fn cols(&self) -> usize {
        self.board.cols()
    }

    pub fn rows(&self) -> usize {
        self.board.rows()
    }

    pub fn state(&self) -> State {
        iproduct!(0..self.cols(), 0..self.rows())
            .map(|(col, row)| match self.board.token_at(col, row) {
                None => 0,
                Some(Token::Player1) => 1,
                Some(Token::Player2) => 2,
            })
            .collect()
    }

    pub fn board(&self) -> Board {
        self.board.clone()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Game {
    pub fn winner(&self) -> Option<(Token, Box<[(usize, usize)]>)> {
        self.winner.clone()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn ctor(cols: Option<usize>, rows: Option<usize>, win_length: Option<usize>) -> Result<Game, JsValue> {
        if cols == None && rows == None && win_length == None {
            Ok(Game::new())
        } else if let (Some(c), Some(r), Some(l)) = (cols, rows, win_length) {
            Ok(Game::custom(c, r, l))
        } else if let (Some(c), Some(r), None) = (cols, rows, win_length) {
            Ok(Game::custom(c, r, 4))
        } else {
            Err("invalid arguments".into())
        }
    }

    #[wasm_bindgen(js_name = "valid_moves")]
    pub fn valid_moves_wasm(&self) -> Box<[JsValue]> {
        self.valid_moves().into_iter()
            .map(|col| JsValue::from(col as u32))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn winner(&self) -> Option<Token> {
        let (token, _) = self.winner.clone()?;
        Some(token)
    }

    pub fn winner_cells(&self) -> Option<Box<[JsValue]>> {
        let (_, cells) = self.winner.clone()?;
        Some(
            cells.iter()
                .map(|(col, row)| JsValue::from_serde(&vec![*col, *row]).unwrap())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    #[wasm_bindgen(js_name = "drop")]
    pub fn drop_wasm(&mut self, col: usize) -> Result<usize, JsValue> {
        match self.drop(col) {
            Err(err) => Err(format!("{}", err).into()),
            Ok(row) => Ok(row),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use self::rand::Rng;

    fn print_search_ranges(cols: usize, rows: usize, wins: usize, col: usize, row: usize) {
        let mut game = Game::custom(cols, rows, wins);

        game.board.highlight(col, row);
        for range in game.search_ranges(col, row) {
            let mut board = game.board.clone();
            for (col, row) in range {
                print!("({},{}) ", col, row);
                board.fill(col, row, Token::Player1).unwrap();
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