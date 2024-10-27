pub const ROW1: u16 = 0b_111_000_000;
pub const ROW2: u16 = 0b_000_111_000;
pub const ROW3: u16 = 0b_000_000_111;

pub const COL1: u16 = 0b_100_100_100;
pub const COL2: u16 = 0b_010_010_010;
pub const COL3: u16 = 0b_001_001_001;

pub const DIA1: u16 = 0b_100_010_001;
pub const DIA2: u16 = 0b_001_010_100;

pub const FILLED_BOARD: u16 = 0b_111_111_111;

pub const WINNING_POSITIONS: [u16; 8] = [COL1, COL2, COL3, ROW1, ROW2, ROW3, DIA1, DIA2];

/// Generates all possible tic-tac-toe moves by iterating over each of the bits and returning bits that dont have a space.
pub fn generate_moves(board: u16) -> Vec<u16> {
    (0..9)
        .map(|i| 1u16 << i)
        .filter(|mask| board & mask == 0)
        .collect()
}

pub fn validate_cell(board: u16, cell: u16) -> bool {
    let positions = generate_moves(board);
    positions.contains(&cell)
}

/// Evaluates the position -10 is a win for agent O 10 is a win for agent X and 0 is a inconclusive
pub fn evaluate_board(agent_x: u16, agent_o: u16) -> i32 {
    for pos in WINNING_POSITIONS {
        // If the AND of a winning position and the players board is true then they must have a winning position
        if agent_x & pos == pos {
            return 10;
        }

        if agent_o & pos == pos {
            return -10;
        }
    }
    0
}

pub fn is_game_draw(board: u16) -> bool {
    board == FILLED_BOARD
}

pub fn board_to_vec(agent_x: u16, agent_o: u16) -> Vec<Vec<char>> {
    (0..9)
        .map(|offset| 1u16 << offset)
        .map(|mask| {
            if agent_x & mask == mask {
                'x'
            }
            else if agent_o & mask == mask {
                'o'
            }
            else {
                ' '
            }
        })
        .collect::<Vec<char>>()
        .chunks_exact(3)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<char>>>()
}