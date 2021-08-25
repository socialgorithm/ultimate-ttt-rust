pub trait GameState:Clone + Copy + PartialEq + Eq + Default {
  fn hash_value(&self) -> usize;
  fn allow_win(&self) -> bool;
  fn to_board_state(&self) -> BoardState;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerState {
    Agent = 1,
    Empty = 0,
    Opponent = 2,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Empty
    }
}

impl GameState for PlayerState {
    fn hash_value(&self) -> usize {
        *self as usize
    }

    fn allow_win(&self) -> bool {
        if self == &PlayerState::Empty {
            false
        } else {
            true
        }
    }

    fn to_board_state(&self) -> BoardState {
        match self {
            PlayerState::Agent => BoardState::Agent,
            PlayerState::Empty => BoardState::Undecided,
            PlayerState::Opponent => BoardState::Opponent,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardState {
    Undecided = 0,
    Agent = 1,
    Opponent = 2,
    Tie = 3,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState::Undecided
    }
}

impl GameState for BoardState {
    fn hash_value(&self) -> usize {
        *self as usize
    }

    fn allow_win(&self) -> bool {
        match self {
            &BoardState::Undecided => false,
            &BoardState::Tie => false,
            _ => true
        }
    }

    fn to_board_state(&self) -> BoardState {
        *self
    }
    
}

