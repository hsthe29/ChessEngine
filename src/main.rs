// mod api;
// mod algorithm;

use shakmaty::{Chess, Position};
use shakmaty::{Square, Move, Role};

// use crate::algorithm::minimax_with_alpha_beta;


fn main() {
    println!("Hello, world!");
    
    let pos = Chess::default();
    let legals = pos.legal_moves();
    
    println!("{:?}", legals);

    let pos = pos.play(&Move::Normal {
        role: Role::Pawn,
        from: Square::E2,
        to: Square::E4,
        capture: None,
        promotion: None,
    });

    println!("{:?}", pos);
}
