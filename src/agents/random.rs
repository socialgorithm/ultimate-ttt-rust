use super::Agent;
use rand::prelude::*;
use crate::game_engine::{LargeBoard, Move, PlayerState};

pub struct RandomAgent {
    game: LargeBoard,
    rng:ThreadRng
}


impl Agent for RandomAgent {
    fn new() -> Self {
        Self {
            game:LargeBoard::new(),
            rng:rand::thread_rng()
        }
    }

    fn init(&mut self) {
        self.game = LargeBoard::new();
    }

    fn make_move(&mut self) -> Move {
        let location = *self.game.get_valid_moves().choose(&mut self.rng).unwrap();
        self.game.make_move(PlayerState::Agent, location);
        location
    }

    fn opponent_move(&mut self, location: Move) {
        self.game.make_move(PlayerState::Opponent, location)
    }
}