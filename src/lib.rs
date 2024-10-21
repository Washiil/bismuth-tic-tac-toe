/*
0   0   0
0   0   0
0   0   0
*/

const ROW1: u16 = 0b_111_000_000;
const ROW2: u16 = 0b_000_111_000;
const ROW3: u16 = 0b_000_000_111;

const COL1: u16 = 0b_100_100_100;
const COL2: u16 = 0b_010_010_010;
const COL3: u16 = 0b_001_001_001;

const DIA1: u16 = 0b_100_010_001;
const DIA2: u16 = 0b_001_010_100;

const FILLED_BOARD: u16 = 0b_111_111_111;

const WINNING_POSITIONS: [u16; 8] = [COL1, COL2, COL3, ROW1, ROW2, ROW3, DIA1, DIA2];

struct TicTacToe {
    board: u16,
    player_x: u16,
    player_0: u16,
    turn: u8,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: 0b000000000,
            player_x: 0b000000000,
            player_0: 0b000000000,
            turn: 0,
        }
    }

    fn evaluate_position(&self) -> i8 {
        for pos in WINNING_POSITIONS {
            // If the AND of a winning position and the players board is true then they must have a winning position
            if self.player_0 & pos == pos {
                return -1;
            }

            if self.player_x & pos == pos {
                return 1;
            }
        }
        0
    }

    pub fn generate_moves(&self) -> Vec<u16> {
        todo!()
    }

    // Our minimax Function 
    pub fn get_best_move(&self) -> u16 {
        todo!()
    }

    fn is_draw(&self) -> bool {
        self.board == FILLED_BOARD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitboard_test() {}
}
