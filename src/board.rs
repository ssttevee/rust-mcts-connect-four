#[cfg(not(target_arch = "wasm32"))]
extern crate termion;

use std::error;
use std::fmt;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, PartialEq)]
pub enum Token {
    Player1,
    Player2,
}

impl Token {
    #[cfg(not(target_arch = "wasm32"))]
    fn color(&self) -> &dyn termion::color::Color {
        use self::Token::{Player1, Player2};

        match self {
            Player1 => &termion::color::Red,
            Player2 => &termion::color::Green,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Copy, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Colored,
    Filled(Token),
    Highlighted(Token),
}

#[cfg(target_arch = "wasm32")]
#[derive(Copy, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Filled(Token),
}

#[cfg(not(target_arch = "wasm32"))]
impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CellState::*;
        use termion::color::{Bg, Fg, Yellow, Reset};

        match self {
            Empty => write!(f, " "),
            Colored => write!(f, "{} {}", Bg(Yellow), Bg(Reset)),
            Filled(token) => write!(f, "{}\u{25CF}{}", Fg(token.color()), Fg(Reset)),
            Highlighted(token) => write!(f, "{}{}{}", Bg(Yellow), Filled(token.to_owned()), Bg(Reset)),
        }
    }
}

#[derive(Debug)]
pub struct InvalidColumnError;

impl fmt::Display for InvalidColumnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid column")
    }
}

impl error::Error for InvalidColumnError {}

#[derive(Debug)]
pub struct AlreadyFilledError;

impl fmt::Display for AlreadyFilledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "already filled")
    }
}

impl error::Error for AlreadyFilledError {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Board {
    cols: usize,
    rows: usize,

    cells: Box<[Box<[CellState]>]>,
}

impl Board {
    pub fn top(&self, col: usize) -> Option<usize> {
        for i in 0..self.rows {
            match self.cells[col][i] {
                CellState::Empty => return Some(i),
                _ => (),
            }
        };

        None
    }

    pub fn drop(&mut self, column: usize, token: Token) -> Result<usize, InvalidColumnError> {
        if self.cols() - 1 < column {
            return Err(InvalidColumnError)
        };
        
        let row = match self.top(column) {
            None => return Err(InvalidColumnError),
            Some(row) => row,
        };

        self.cells[column][row] = CellState::Filled(token);

        Ok(row)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Board {
    pub fn fill(&mut self, col: usize, row: usize, token: Token) -> Result<(), AlreadyFilledError> {
        self.cells[col][row] = match self.cells[col][row] {
            CellState::Empty => CellState::Filled(token),
            CellState::Colored => CellState::Highlighted(token),
            _ => return Err(AlreadyFilledError),
        };

        Ok(())
    }
    
    pub fn highlight(&mut self, col: usize, row: usize) {
        self.cells[col][row] = match self.cells[col][row] {
            CellState::Empty => CellState::Colored,
            CellState::Filled(ref token) => CellState::Highlighted(token.to_owned()),
            _ => return
        }
    }
    
    pub fn token_at(&self, col: usize, row: usize) -> Option<Token> {
        match self.cells[col][row] {
            CellState::Empty | CellState::Colored => None,
            CellState::Filled(ref p) | CellState::Highlighted(ref p) => Some(p.clone()),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Board {
    pub fn fill(&mut self, col: usize, row: usize, token: Token) -> Result<(), AlreadyFilledError> {
        self.cells[col][row] = match self.cells[col][row] {
            CellState::Empty => CellState::Filled(token),
            _ => return Err(AlreadyFilledError),
        };

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Board {
    pub fn token_at(&self, col: usize, row: usize) -> Option<Token> {
        match self.cells[col][row] {
            CellState::Empty => None,
            CellState::Filled(ref p) => Some(p.clone()),
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(js_name = "fill")]
    pub fn fill_wasm(&mut self, col: usize, row: usize, token: Token) -> Result<(), JsValue> {
        self.cells[col][row] = match self.cells[col][row] {
            CellState::Empty => CellState::Filled(token),
            _ => return Err("already filled".into()),
        };

        Ok(())
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Board {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(cols: usize, rows: usize) -> Board {
        Board {
            cols: cols,
            rows: rows,
            cells: vec![vec![CellState::Empty; rows].into_boxed_slice(); cols].into_boxed_slice()
        }
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
}

impl std::clone::Clone for Board {
    fn clone(&self) -> Board {
        Board {
            cols: self.cols,
            rows: self.rows,
            cells: self.cells.to_vec().iter()
                .map(|row| row.clone())
                .collect::<Vec<_>>().into_boxed_slice()
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct Row<'a>(&'a Board, usize);

#[cfg(not(target_arch = "wasm32"))]
impl <'a> fmt::Display for Row<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.0.cols - 1 {
            write!(f, "{}", self.0.cells[i][self.1])?;
            f.write_str(" | ")?;
        }
        write!(f, "{}", self.0.cells[self.0.cols - 1][self.1])
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row_sep = (0..self.cols).map(|_| "----").collect::<Vec<_>>().join("");
        for i in (0..self.rows).rev() {
            writeln!(f, "-{}", row_sep)?;
            writeln!(f, "| {} |", Row(self, i))?;
        }
        write!(f, "-{}", row_sep)
    }
}
