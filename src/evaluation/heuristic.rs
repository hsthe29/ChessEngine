use std::collections::HashMap;

use shakmaty::{Chess, Position};
use shakmaty::{Square, Move, Role, Color};


const ALPHA: i32 = std::i32::MIN;
const BETA: i32 = std::i32::MAX;

pub struct Heuristic {
    piece_value: HashMap<Role, i32>,
    w_position_table: HashMap<Role, [[i32; 8]; 8]>,
    b_position_table: HashMap<Role, [[i32; 8]; 8]>
}

impl Heuristic {
    pub const fn new() -> Heuristic {
        Heuristic {

        }
    }
}

const PIECE_VALUE: HashMap<Role, i32> = HashMap::from([
    (Role::Pawn, 100),
    (Role::Knight, 280),
    (Role::Bishop, 320),
    (Role::Rook, 479),
    (Role::Queen, 929),
    (Role::King, 60000)
]);

const white_position_score: HashMap<Role, [[i32; 8]; 8]> = HashMap::from([
    (Role::Pawn, [
        [100, 100, 100, 100, 105, 100, 100, 100],
        [78, 83, 86, 73, 102, 82, 85, 90],
        [7, 29, 21, 44, 40, 31, 44, 7],
        [-17, 16, -2, 15, 14, 0, 15, -13],
        [-26, 3, 10, 9, 6, 1, 0, -23],
        [-22, 9, 5, -11, -10, -2, 3, -19],
        [-31, 8, -7, -37, -36, -14, 3, -31],
        [0, 0, 0, 0, 0, 0, 0, 0]]
    ),
    (Role::Knight, [
        [-66, -53, -75, -75, -10, -55, -58, -70],
        [-3, -6, 100, -36, 4, 62, -4, -14],
        [10, 67, 1, 74, 73, 27, 62, -2],
        [24, 24, 45, 37, 33, 41, 25, 17],
        [-1, 5, 31, 21, 22, 35, 2, 0],
        [-18, 10, 13, 22, 18, 15, 11, -14],
        [-23, -15, 2, 0, 2, 0, -23, -20],
        [-74, -23, -26, -24, -19, -35, -22, -69]]
    ),
    (Role::Bishop, [
        [-59, -78, -82, -76, -23, -107, -37, -50],
        [-11, 20, 35, -42, -39, 31, 2, -22],
        [-9, 39, -32, 41, 52, -10, 28, -14],
        [25, 17, 20, 34, 26, 25, 15, 10],
        [13, 10, 17, 23, 17, 16, 0, 7],
        [14, 25, 24, 15, 8, 25, 20, 15],
        [19, 20, 11, 6, 7, 6, 20, 16],
        [-7, 2, -15, -12, -14, -15, -10, -1]]
    ),
    (Role::Rook, [
        [35, 29, 33, 4, 37, 33, 56, 50],
        [55, 29, 56, 67, 55, 62, 34, 60],
        [19, 35, 28, 33, 45, 27, 25, 15],
        [0, 5, 16, 13, 18, -4, -9, -6],
        [-28, -35, -16, -21, -13, -29, -46, -30],
        [-42, -28, -42, -25, -25, -35, -26, -46],
        [-53, -38, -31, -26, -29, -43, -44, -53],
        [-30, -24, -18, 5, -2, -18, -31, -3]]
    ),
    (Role::Queen, [
        [6, 1, -8, -104, 69, 24, 88, 26],
        [14, 32, 60, -10, 20, 76, 57, 24],
        [-2, 43, 32, 60, 72, 63, 43, 2],
        [1, -16, 22, 17, 25, 20, -13, -6],
        [-14, -15, -2, -5, -1, -10, -20, -22],
        [-30, -6, -13, -11, -16, -11, -16, -27],
        [-36, -18, 0, -19, -15, -15, -21, -38],
        [-39, -30, -31, -13, -31, -36, -34, -4]]
    ),
    (Role::King, [
        [4, 54, 47, -99, -99, 60, 83, -62],
        [-32, 10, 55, 56, 56, 55, 10, 3],
        [-62, 12, -57, 44, -67, 28, 37, -31],
        [-55, 50, 11, -4, -19, 13, 0, -49],
        [-55, -43, -52, -28, -51, -47, -8, -50],
        [-47, -42, -43, -79, -64, -32, -29, -32],
        [-4, 3, -14, -50, -57, -18, 13, 4],
        [17, 30, -3, -14, 6, -1, 40, 18]]
    )
]);

const black_position_score: HashMap<Role, [[i32; 8]; 8]> = HashMap::from(

);

/*
 * Evaluates the board at this point in time,
 * using the material weights and piece square tables.
 */

fn get_position_score(engine: &Chess) -> i32 {
    let mut score = 0;
    for sq in Square::ALL {
        score += PIECE_VALUE[sq];
    }
    return sq;
}

fn get_material_score(state: &Chess) -> i32 {
    let board = state.board();
    let turn = state.turn();
    let mut score = 0;

    let position_score_table = if turn == Color::White { white_position_score } else { black_position_score };

    for sq in Square::ALL {
        let piece = board.piece_at(sq);
        score += position_score_table[piece.unwrap().role];
    }
    return score
}

pub fn evaluate_state(state: &Chess) -> i32 {
    return get_position_score(state) + get_material_score(state);
}

fn checkStatus(state: &Chess) -> bool {
    if state.is_checkmate() {
        println!("Checkmate!");
    } 
    else if state.is_insufficient_material() {
        println!("It's a draw! (Insufficient Material)");
    }
    else if state.is_stalemate() {
        println!("It's a draw! (Stalemate)");
    } 
    else if state.outcome() {
        println!("It's a draw! (50-move Rule)");
    }
    else {
        println("No check, checkmate, or draw.");
    }
    engine.endGame.set(true);
    return true
}