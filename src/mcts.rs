extern crate rand;

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use game::{Game, State};

use rand::Rng;
use rand::distributions::{WeightedIndex, Distribution};
use rand::prelude::ThreadRng;

pub struct MCTS {
    memory: HashMap<State, Vec<(f64, usize)>>,
}

fn winrate((score, games): (f64, usize)) -> f64 {
    if games == 0 {
        0.5
    } else {
        score / games as f64
    }
}

impl MCTS {
    fn update(&mut self, score: f64, moves: Vec<(State, usize)>) {
        // the distance to a leaf is inversely proportional to the score
        // i.e. the closer to a winning or losing move, the greater the effect on
        // the next choice of move
        for (i, (state, m)) in moves.iter().rev().enumerate() {
            let record = self.memory.get_mut(state).unwrap().get_mut(*m).unwrap();
            record.0 += 0.5 + score * 0.5 / (i + 1) as f64;
            record.1 += 1;
        }
    }

    pub fn move_weights(&self, state: &State, moves: &Vec<usize>) -> Vec<f64> {
        let records = &self.memory[state];

        moves.iter()
            .map(|col| winrate(records[col - 1]))
            .collect::<Vec<_>>()
    }

    fn pick_move(&self, rng: &mut ThreadRng, state: &State, choices: &Vec<usize>) -> usize {
        let weights = self.move_weights(state, choices);
        if weights.iter().all(|w| *w == 0.0) {
            return choices[rng.gen_range(0, choices.len())]
        }

        let dist = WeightedIndex::new(&weights).unwrap();
        choices[dist.sample(rng)]
    }

    fn simulate(&mut self, rng: &mut ThreadRng, mut game: Game) -> (usize, usize, usize) {
        let mut my_moves: Vec<(State, usize)> = Vec::new();
        let mut their_moves: Vec<(State, usize)> = Vec::new();

        let cols = game.cols(); // cache this

        // assume current player is positive
        let mut i = 0;
        while !game.over() {
            // selection
            let state = game.state();
            if !self.memory.contains_key(&state) {
                // expansion
                self.memory.insert(state.clone(), vec![(0.0,0); cols]);
            }

            let col = self.pick_move(rng, &state, &game.valid_moves());

            // keep track of each players' moves
            if i % 2 == 0 {
                &mut my_moves
            } else {
                &mut their_moves
            }.push((state, col - 1));

            game.drop(col).unwrap();

            i += 1;
        } // simulation ends on the loser's turn

        // back propagation
        let (my_score, their_score, results) = if game.winner() == None { // tie
            (0.0, 0.0, (0,0,1))
        } else if i % 2 == 0 { // lost
            (-1.0, 1.0, (0,1,0))
        } else { // won
            (1.0, -1.0, (1,0,0))
        };

        self.update(my_score as f64, my_moves);
        self.update(their_score as f64, their_moves);

        results
    }

    pub fn think(&mut self, rng: &mut ThreadRng, game: &Game, duration: Duration) -> [usize; 4] {
        let now = SystemTime::now();
        let mut results = [0 as usize; 4];
        while now.elapsed().unwrap() < duration {
            let (wins, losses, ties) = self.simulate(rng, game.to_owned());
            results[0] += wins;
            results[1] += losses;
            results[2] += ties;
            results[3] += 1;
        }
        
        results
    }
}

pub fn new() -> MCTS {
    MCTS { memory: HashMap::new() }
}
