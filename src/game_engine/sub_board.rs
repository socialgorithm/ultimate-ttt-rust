use super::enums::{BoardState, GameState};
use super::types::Coord;

const SIZE: usize = 3;
const MOVE_LIMIT: usize = SIZE.pow(2);

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct SubBoard<T> {
    board: [[T; 3]; 3],
    winner: Option<BoardState>,
    move_count: usize
}

impl<T> SubBoard<T> where T:GameState {
    pub fn new() -> Self {
        Self {
            board: [[T::default(); 3]; 3],
            winner: None,
            move_count: 0
        }
    }

    pub fn make_move(&mut self, player: T, location: Coord) {
        if self.is_finished() {
            panic!("Attempted to make move on finished board");
        }
        self.board[location.x][location.y] = player;
        self.move_count += 1;

        // Check if the board has been won
        self.check_row(location.x);

        if !self.is_finished() {
            self.check_column(location.y);
        }

        if !self.is_finished() {
            self.check_ltr_diagonal();
        }

        if !self.is_finished() {
            self.check_rtl_diagonal();
        }

        // check for a tie
        if !self.is_finished() && self.move_count == MOVE_LIMIT {
            self.winner = Some(BoardState::Tie);
        }
    }

    pub fn is_finished(&self) -> bool {
        self.winner.is_some()
    }

    pub fn hash(&self) -> usize {
        let mut hash = 0;
        for square in self.board.iter().flatten() {
            hash += square.hash_value();
            hash = hash << 2;
        }
        hash
    }

    pub fn undo_move(&mut self, location: Coord) {
        self.board[location.x][location.y] = T::default();
        self.move_count -= 1;
        self.winner = None;
    }

    pub fn get_valid_moves(&self) -> Vec<Coord> {
        self.board
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, v)| **v == T::default())
            .map(|(i, _)| Coord::new(i / 3, i % 3))
            .collect()
    }

    pub fn is_winner(&self) -> Option<BoardState> {
        self.winner
    }

    fn check_row(&mut self, row: usize) {
        let player = self.board[row][0];
        if !player.allow_win() {
            return;
        }
        for i in 1..SIZE {
            if player != self.board[row][i] {
                return;
            }
        }
        self.winner = Some(player.to_board_state())
    }

    /**
     * Check if a given column has been won
     * @param col Column index
     */
    fn check_column(&mut self, col: usize) {
        let player = self.board[0][col];
        if !player.allow_win() {
            return;
        }
        for i in 1..SIZE {
            if player != self.board[i][col] {
                return;
            }
        }
        self.winner = Some(player.to_board_state())
    }

    fn check_ltr_diagonal(&mut self) {
        let player = self.board[0][0];
        if !player.allow_win() {
            return;
        }
        for i in 1..SIZE {
            if player != self.board[i][i] {
                return;
            }
        }
        self.winner = Some(player.to_board_state())
    }

    /**
     * Check if the right to left diagonal has been won
     */
    fn check_rtl_diagonal(&mut self) {
        let player = self.board[SIZE - 1][0];
        if !player.allow_win() {
            return;
        }
        for i in 1..SIZE {
            if player != self.board[SIZE - 1 - i][i] {
                return;
            }
        }
        self.winner = Some(player.to_board_state())
    }
}
