use std::cmp::Ordering;
use shakmaty::{Chess, Position};

use super::evaluation::heuristic;

pub fn minimax_with_alpha_beta(
    state: &Chess,
    alpha: i32,
    beta: i32,
    depth: i32,
    maximizing_player: bool,
) -> (Option<Chess>, i32) {
    if depth == 0 {
        return heuristic::evaluate_state(state);
    }

    let legal_moves = state.legal_moves();

    let mut best_move = None;
    let mut best_value = std::i32::MIN;

    for legal_move in legal_moves {
        let new_state = state.play(legal_move);

        let (_best_move, value) = minimax_with_alpha_beta(
            new_state,
            alpha,
            beta,
            depth - 1,
            !maximizing_player,
        );

        if maximizing_player {
            if value > best_value {
                best_value = value;
                best_move = Some(legal_move);
            }
            alpha = std::cmp::max(alpha, value);
        } else {
            if value < best_value {
                best_value = value;
                best_move = Some(legal_move);
            }
            beta = std::cmp::min(beta, value);
        }

        if alpha >= beta {
            break;
        }
    }

    return (best_move, best_value);
}
