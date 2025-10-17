// Engine module with placeholder for MCTS interface

use crate::game::GameState;
use ::chess::{Board, BoardStatus, ChessMove, MoveGen};
use rand::seq::SliceRandom;
// use rand::Rng; // not needed currently
use std::collections::HashMap;
use std::f32::consts::SQRT_2;

pub struct MctsConfig {
    pub simulations: usize,
}

impl Default for MctsConfig {
    fn default() -> Self {
        Self { simulations: 1000 }
    }
}

pub struct MctsEngine {
    pub config: MctsConfig,
}

impl MctsEngine {
    pub fn new(config: MctsConfig) -> Self {
        Self { config }
    }

    pub fn choose_move(&self, state: &GameState) -> Option<ChessMove> {
        let legal: Vec<ChessMove> = state.legal_moves();
        if legal.is_empty() { return None; }
        if legal.len() == 1 { return Some(legal[0]); }

        let mut rng = rand::thread_rng();
        let root_board = state.board;
        let mut root = Node::new(root_board);

        let simulations = self.config.simulations.max(1);
        for _ in 0..simulations {
            // 1) Selection
            let mut path: Vec<*mut Node> = Vec::new();
            let mut node_ptr: *mut Node = &mut root;
            unsafe {
                loop {
                    path.push(node_ptr);
                    let node = &mut *node_ptr;
                    if node.is_terminal {
                        break;
                    }
                    if node.unexpanded_moves.is_empty() && !node.children.is_empty() {
                        // Select child with highest UCT
                        let parent_visits = node.visits.max(1) as f32;
                        let mut best_key: Option<ChessMove> = None;
                        let mut best_score = f32::NEG_INFINITY;
                        for (mv, child) in node.children.iter_mut() {
                            let uct = child.uct_value(parent_visits);
                            if uct > best_score {
                                best_score = uct;
                                best_key = Some(*mv);
                            }
                        }
                        if let Some(mv) = best_key {
                            node_ptr = node.children.get_mut(&mv).unwrap().as_mut();
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                // 2) Expansion (if possible)
                let expanded = {
                    let node = &mut *node_ptr;
                    if node.is_terminal {
                        None
                    } else if let Some(mv) = node.unexpanded_moves.pop() {
                        let next_board = node.board.make_move_new(mv);
                        let child = Node::new(next_board);
                        node.children.insert(mv, Box::new(child));
                        Some(mv)
                    } else {
                        None
                    }
                };

                if let Some(mv) = expanded {
                    // move node_ptr to the expanded child
                    let node = &mut *node_ptr;
                    node_ptr = node.children.get_mut(&mv).unwrap().as_mut();
                    path.push(node_ptr);
                }

                // 3) Simulation
                let sim_result = {
                    let node = &mut *node_ptr;
                    simulate_random_default(&node.board, 96, &mut rng)
                };

                // 4) Backpropagation
                for &p in path.iter() {
                    let node = &mut *p;
                    node.visits += 1;
                    node.value_sum += sim_result;
                }
            }
        }

        // Pick the child with most visits
        if root.children.is_empty() {
            return legal.into_iter().next();
        }
        let mut best_move: Option<(ChessMove, u32)> = None;
        for (mv, child) in root.children.into_iter() {
            match best_move {
                Some((_, best_vis)) if child.visits <= best_vis => {}
                _ => best_move = Some((mv, child.visits)),
            }
        }
        best_move.map(|(mv, _)| mv)
    }
}

fn simulate_random_default(start: &Board, max_depth: usize, rng: &mut rand::rngs::ThreadRng) -> f32 {
    // Return +1 if the starting side eventually wins, -1 if loses, 0 draw/unknown
    let start_side = start.side_to_move();
    let mut board = *start;
    for _ in 0..max_depth {
        match board.status() {
            BoardStatus::Ongoing => {
                let mut moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();
                if moves.is_empty() { break; }
                moves.shuffle(rng);
                board = board.make_move_new(moves[0]);
            }
            BoardStatus::Checkmate => {
                let winner = !board.side_to_move();
                return if winner == start_side { 1.0 } else { -1.0 };
            }
            BoardStatus::Stalemate => return 0.0,
        }
    }
    0.0
}

#[derive(Debug)]
struct Node {
    board: Board,
    visits: u32,
    value_sum: f32,
    children: HashMap<ChessMove, Box<Node>>,
    unexpanded_moves: Vec<ChessMove>,
    is_terminal: bool,
}

impl Node {
    fn new(board: Board) -> Self {
        let status = board.status();
        let is_terminal = !matches!(status, BoardStatus::Ongoing);
        let unexpanded_moves = if is_terminal {
            Vec::new()
        } else {
            MoveGen::new_legal(&board).collect::<Vec<ChessMove>>()
        };
        Self {
            board,
            visits: 0,
            value_sum: 0.0,
            children: HashMap::new(),
            unexpanded_moves,
            is_terminal,
        }
    }

    fn uct_value(&self, parent_visits: f32) -> f32 {
        if self.visits == 0 {
            return f32::INFINITY;
        }
        let mean_value = self.value_sum / self.visits as f32;
        let exploration = SQRT_2 * ((parent_visits.ln() / self.visits as f32).sqrt());
        mean_value + exploration
    }
}


