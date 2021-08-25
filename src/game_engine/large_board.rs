use super::enums::{PlayerState, BoardState};
use super::sub_board::SubBoard;
use super::types::{Coord, Move};

const SIZE: usize = 3;

#[derive(Clone,Debug)]
pub struct LargeBoard {
    board: [[SubBoard<PlayerState>; 3]; 3],
    previous_turns: Vec<Move>,
    state_board: SubBoard<BoardState>,
    move_count:usize
}

impl LargeBoard {
    pub fn new() -> Self {
        Self {
            board: [[SubBoard::new(); 3]; 3],
            previous_turns: Vec::new(),
            state_board: SubBoard::new(),
            move_count:0
        }
    }

    pub fn make_move(&mut self, player: PlayerState, location: Move) {
        self.move_count += 1;
        let sub_board = &mut self.board[location.large_board.x][location.large_board.y];
        sub_board.make_move(player, location.sub_board);

        if sub_board.is_finished() {
            self.state_board.make_move(sub_board.is_winner().unwrap(), location.large_board);
        }

        self.previous_turns.push(location);
    }

    pub fn undo_move(&mut self) {
        self.move_count -= 1;
        let last_move = self
            .previous_turns
            .pop()
            .expect("Can not undo move in empty game");
        let sub_board = &mut self.board[last_move.large_board.x][last_move.large_board.y];
        let was_finished = sub_board.is_finished();
        sub_board.undo_move(last_move.sub_board);

        if was_finished {
            self.state_board.undo_move(last_move.large_board);
        }
    }

    pub fn get_next_board(&self) -> Option<Coord> {
        self.previous_turns.last().map(|m| m.sub_board)
    }

    pub fn get_valid_boards(&self) -> Vec<Coord> {
        let next_board = self.get_next_board();
        if next_board.is_some() {
            let next_board = next_board.unwrap();
            if !self.board[next_board.x][next_board.y].is_finished() {
                return vec![next_board];
            }
        }
        self.board
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, sub_board)| !sub_board.is_finished())
            .map(|(i, _)| Coord::new(i / SIZE, i % SIZE))
            .collect()
    }

    pub fn get_valid_moves(&self) -> Vec<Move> {
        if self.state_board.is_finished() {
            return Vec::new()
        }
        self.get_valid_boards()
            .into_iter()
            .flat_map(|large_board_cords| {
                self.board[large_board_cords.x][large_board_cords.y]
                    .get_valid_moves()
                    .into_iter()
                    .map(move |sub_board_cords| Move::new(large_board_cords, sub_board_cords))
            })
            .collect()
    }

    pub fn state_board(&self) -> &SubBoard<BoardState> {
        &self.state_board
    }

    pub fn board(&self) -> &[[SubBoard<PlayerState>; 3]; 3] {
        &self.board
    }

    pub fn move_count(&self) -> usize {
        self.move_count
    }

    pub fn is_winner(&self) -> Option<BoardState> {
        self.state_board.is_winner()
    }
}
