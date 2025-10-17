// Engine module with placeholder for MCTS interface

use crate::chess::GameState;
use chess::{Board, BoardStatus, ChessMove, MoveGen};
use rand::seq::SliceRandom;

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
        // Minimal Monte Carlo: sample random playouts per legal move and pick best by win rate
        let legal: Vec<ChessMove> = state.legal_moves();
        if legal.is_empty() { return None; }

        // If only one, short-circuit
        if legal.len() == 1 { return Some(legal[0]); }

        let mut rng = rand::thread_rng();
        let mut best: Option<(ChessMove, f32)> = None;
        for mv in legal.into_iter() {
            let mut total_score = 0.0f32;
            let sims = self.config.simulations.max(1);
            for _ in 0..sims {
                let score = random_playout(state.board.make_move_new(mv), 64, &mut rng);
                total_score += score;
            }
            let avg = total_score / sims as f32;
            match best {
                Some((_, best_avg)) if avg <= best_avg => {}
                _ => best = Some((mv, avg)),
            }
        }
        best.map(|(mv, _)| mv)
    }
}

fn random_playout(mut board: Board, max_depth: usize, rng: &mut rand::rngs::ThreadRng) -> f32 {
    // Return +1 if side to move at start eventually wins, -1 if loses, 0 draw
    let start_side = board.side_to_move();
    for _ in 0..max_depth {
        if !matches!(board.status(), BoardStatus::Ongoing) {
            break;
        }
        let mut moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();
        if moves.is_empty() { break; }
        moves.shuffle(rng);
        board = board.make_move_new(moves[0]);
    }
    match board.status() {
        BoardStatus::Checkmate => {
            // Winner is the side who just moved; since status is after playout, infer by flipping
            let winner = !board.side_to_move();
            if winner == start_side { 1.0 } else { -1.0 }
        }
        BoardStatus::Stalemate => 0.0,
        BoardStatus::Ongoing => 0.0,
    }
}


