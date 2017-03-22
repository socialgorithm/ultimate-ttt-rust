pub mod tic_tac_toe {
    const ME: u8 = 0;
    const OPPONENT: u8 = 1;

    const RESULT_TIE: i8 = -1;
    const RESULT_WIN: i8 = 0;
    const RESULT_LOSE: i8 = 1;

    pub struct Cell {
        player: u8,
        sub_board_index: u32,
        main_index: u32
    }

    pub struct tic_tac_toe {
        //board: Vec<Vec<Cell>>,
        moves: u8,
        winner: i8,
    }

    impl tic_tac_toe {
        pub fn new(size: u8) -> tic_tac_toe {
            tic_tac_toe {
                moves: 0,
                winner: RESULT_TIE
            }
        }

        pub fn print(&self) {
            println!("TicTacToe");
            println!("Moves: {}", self.moves)
        }
    }
}