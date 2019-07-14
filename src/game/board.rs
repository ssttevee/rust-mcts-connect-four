extern crate termion;

use std::error;
use std::fmt;

pub trait Token: Clone + PartialEq + fmt::Display {
    fn color(&self) -> &dyn termion::color::Color;
}

#[derive(Copy, Clone, PartialEq)]
pub enum CellState<T: Token> {
    Empty,
    Colored,
    Filled(T),
    Highlighted(T),
}

impl <T: Token> CellState<T> {
    pub fn has(&self, token: T) -> bool {
        match self {
            CellState::Filled(p) => *p == token,
            _ => false,
        }
    }
}

impl <T: Token> fmt::Display for CellState<T> {
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

pub struct Board<T: Token> {
    cols: usize,
    rows: usize,

    cells: Box<[Box<[CellState<T>]>]>,
}

impl <T: Token> Board<T> {
    fn top(&self, col: usize) -> Option<usize> {
        for i in 0..self.rows {
            match self.cells[col][i] {
                CellState::Empty => return Some(i),
                _ => (),
            }
        };

        None
    }

    pub fn drop(&mut self, mut column: usize, token: T) -> Result<usize, InvalidColumnError> {
        if column < 1 || self.cols() < column {
            return Err(InvalidColumnError)
        };

        column -= 1;
        
        let row = match self.top(column) {
            None => return Err(InvalidColumnError),
            Some(row) => row,
        };

        self.cells[column][row] = CellState::Filled(token);

        Ok(row)
    }

    pub fn fill(&mut self, col: usize, row: usize, token: T) {
        self.cells[col][row] = match self.cells[col][row] {
            CellState::Empty => CellState::Filled(token),
            CellState::Colored => CellState::Highlighted(token),
            _ => return
        }
    }

    pub fn highlight(&mut self, col: usize, row: usize) {
        self.cells[col][row] = match self.cells[col][row] {
            CellState::Empty => CellState::Colored,
            CellState::Filled(ref token) => CellState::Highlighted(token.to_owned()),
            _ => return
        }
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
    
    pub fn token_at(&self, col: usize, row: usize) -> Option<T> {
        match self.cells[col][row] {
            CellState::Empty | CellState::Colored => None,
            CellState::Filled(ref p) | CellState::Highlighted(ref p) => Some(p.clone()),
        }
    }
}

impl <'a, T: Token> std::clone::Clone for Board<T> {
    fn clone(&self) -> Board<T> {
        Board {
            cols: self.cols,
            rows: self.rows,
            cells: self.cells.to_vec().iter()
                .map(|row| row.clone())
                .collect::<Vec<_>>().into_boxed_slice()
        }
    }
}

struct Row<'a, T: Token>(&'a Board<T>, usize);

impl <'a, T: Token> fmt::Display for Row<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.0.cols - 1 {
            write!(f, "{}", self.0.cells[i][self.1])?;
            f.write_str(" | ")?;
        }
        write!(f, "{}", self.0.cells[self.0.cols - 1][self.1])
    }
}

impl <T: Token> fmt::Display for Board<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row_sep = (0..self.cols).map(|_| "----").collect::<Vec<_>>().join("");
        
        writeln!(f, "| {} |", (1..=self.cols).map(|i| i.to_string()).collect::<Vec<_>>().join(" | "))?;
        for i in (0..self.rows).rev() {
            writeln!(f, "-{}", row_sep)?;
            writeln!(f, "| {} |", Row::<T>(self, i))?;
        }
        write!(f, "-{}", row_sep)
    }
}

// impl <T: Token> std::ops::IndexMut<(usize, usize)> for Board<T> {
//     fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
//         &mut self.cells[col][row]
//     }
// }

// impl <T: Token> std::clone::Clone for Board<T> {
//     fn clone(&self) -> Board<T> {

//     }
// }

pub fn new<T: Token>(cols: usize, rows: usize) -> Board<T> {
    Board::<T> {
        cols: cols,
        rows: rows,
        cells: vec![vec![CellState::<T>::Empty; rows].into_boxed_slice(); cols].into_boxed_slice()
    }
}
