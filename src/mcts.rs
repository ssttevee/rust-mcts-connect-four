use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, SystemTime};

use game::{Game, State};

#[cfg(not(target_arch = "wasm32"))]
extern crate rand;

#[cfg(not(target_arch = "wasm32"))]
fn random() -> f64 {
    rand::random()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;

    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> usize;
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct MCTS {
    memory: HashMap<State, Vec<(f64, usize)>>,
}

fn random_weighted(weights: Vec<f64>) -> usize {
    let n = weights.len();
    if n == 0 {
        panic!("no values")
    }

    let sum = weights.iter().fold(0.0, |sum, w| sum + w);
    if sum == 0.0 {
        return (random() * n as f64) as usize
    }

    let value = random();
    let mut progress = 0.0;
    for (i, weight) in weights.iter().enumerate() {
        let normalized = weight / sum;
        if value < progress + normalized {
            return i
        }

        progress += normalized;
    }

    unreachable!()
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

    fn pick_move(&self, state: &State, choices: &Vec<usize>) -> usize {
        choices[random_weighted(self.move_weights(state, choices))]
    }

    pub fn simulate(&mut self, mut game: Game) -> (usize, usize, usize) {
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

            let col = self.pick_move(&state, &game.valid_moves());

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
}

#[cfg(not(target_arch = "wasm32"))]
impl MCTS {
    pub fn think(&mut self, game: &Game, duration: Duration) -> [usize; 4] {
        let now = SystemTime::now();
        let mut results = [0 as usize; 4];
        while now.elapsed().unwrap() < duration {
            let (wins, losses, ties) = self.simulate(game.to_owned());
            results[0] += wins;
            results[1] += losses;
            results[2] += ties;
            results[3] += 1;
        }
        
        results
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MCTS {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> MCTS {
        MCTS { memory: HashMap::new() }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl MCTS {
    #[wasm_bindgen(js_name = "simulate")]
    pub fn simulate_wasm(&mut self, game: &Game) -> Box<[JsValue]> {
        let result = self.simulate(game.clone());
        vec![
            JsValue::from(result.0 as u32),
            JsValue::from(result.1 as u32),
            JsValue::from(result.2 as u32),
        ].into_boxed_slice()
    }

    #[wasm_bindgen(js_name = "think")]
    pub fn think(&mut self, game: &Game, duration: usize) -> Box<[JsValue]> {
        let start = now();
        let mut results = [0 as usize; 4];
        while now() - start < duration {
            let (wins, losses, ties) = self.simulate(game.to_owned());
            results[0] += wins;
            results[1] += losses;
            results[2] += ties;
            results[3] += 1;
        }

        vec![
            JsValue::from(results[0] as u32),
            JsValue::from(results[1] as u32),
            JsValue::from(results[2] as u32),
            JsValue::from(results[3] as u32),
        ].into_boxed_slice()
    }

    #[wasm_bindgen(js_name = "move_weights")]
    pub fn move_weights_wasm(&self, state: State, moves: Box<[JsValue]>) -> Result<Box<[JsValue]>, JsValue> {
        let mut resolved_moves = Vec::<usize>::new();
        for value in moves.iter() {
            if let Some(num) = value.as_f64() {
                resolved_moves.push(num as usize);
            } else {
                return Err("moves must be an array of numbers".into())
            }
        }

        Ok(self.move_weights(&state, &resolved_moves).into_iter()
            .map(|col| JsValue::from(col))
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }
}
