use board::{Board, Token};
use game::{Game, search_ranges};
use common::random;
use std::iter;

fn range_score<'a>(board: &Board, token: Token, range: Box<dyn iter::Iterator<Item = (usize, usize)> + 'a>) -> usize {
    range.fold(0, |sum, (col, row)| sum + match board.token_at(col, row) {
        Some(t) => if t == token {
            1
        } else {
            0
        },
        None => 0,
    })
}

fn calculate_column_score(board: &Board, token: Token, win_len: usize, col: usize) -> usize {
    match board.top(col) {
        Some(row) => search_ranges(board.cols(), board.rows(), win_len, col, row)
            .map(|range| range_score(board, token, range))
            .fold(0, |best, score| if score > best {
                score
            } else {
                score
            }),
        None => 0,
    }
}

pub fn next_move(game: &Game) -> usize {
    let board = game.board();

    let column_scores = (0..game.cols())
        .map(|col| calculate_column_score(&board, game.current_player(), game.win_len(), col))
        .enumerate()
        .collect::<Vec<_>>();

    let best_score = column_scores
        .iter()
        .fold(0, |best_score, (_, score)| if *score > best_score {
            *score
        } else {
            best_score
        });

    let best_columns = column_scores
        .into_iter()
        .filter_map(|(col, score)| if score == best_score {
            Some(col)
        } else {
            None
        })
        .collect::<Vec<_>>();

    if best_columns.len() == 0 {
        (random() * board.cols() as f64) as usize
    } else if best_columns.len() == 1 {
        best_columns[0]
    } else {
        best_columns[(random() * best_columns.len() as f64) as usize]
    }
}