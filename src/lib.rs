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

fn evaluate_position(agent_x: u16, agent_o: u16) -> i8 {
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

pub fn generate_moves(board: u16) -> Vec<u16> {
    let mut possible_moves = Vec::new();
    for pos in 0..9 {
        let mask = 1u16 << pos;

        if board & mask == 0 {
            possible_moves.push(mask);
        }
    }
    possible_moves
}

// Our minimax Function
pub fn minimax(agent_x: u16, agent_o: u16, depth: i32, maximizing_player: bool) -> (i8, u16) {
    let score = evaluate_position(agent_x, agent_o);
    if score != 0 || depth == 0 || is_game_draw(agent_x | agent_o) {
        return (score, 0);
    }

    if maximizing_player {
        let mut best_value = i8::MIN;
        let mut best_move = 0;

        for pos in generate_moves(agent_x | agent_o) {
            let future_eval = minimax(agent_x | pos, agent_o, depth - 1, false);
            if future_eval.0 > best_value {
                best_value = future_eval.0;
                best_move = pos;
            }
        }
        return (best_value, best_move);
    } else {
        let mut best_value = i8::MAX;
        let mut best_move = 0;

        for pos in generate_moves(agent_x | agent_o) {
            let future_eval = minimax(agent_x, agent_o | pos, depth - 1, true);
            if future_eval.0 < best_value {
                best_value = future_eval.0;
                best_move = pos;
            }
        }
        return (best_value, best_move);
    }
}

pub fn get_best_move(agent_x: u16, agent_o: u16, is_x_turn: bool) -> u16 {
    let (_, best_move) = minimax(agent_x, agent_o, 18, is_x_turn);
    best_move
}

fn is_game_draw(board: u16) -> bool {
    board == FILLED_BOARD
}

pub fn print_u16_as_board(agent_x: u16, agent_o: u16) {
    println!("-Board-");
    print!("| ");
    for slot in 0..9 {
        let mask = 1u16 << slot;
        if slot != 0 && slot % 3 == 0 {
            print!(" |\n| ");
        }

        if agent_x & mask > 0 {
            print!("X")
        } else if agent_o & mask > 0 {
            print!("O")
        } else {
            print!(" ")
        }
    }
    println!(" |\n-------")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitboard_test() {
        let mut agent_x: u16 = 0b_000_000_000;
        let mut agent_o: u16 = 0b_000_000_000;
        let mut turn = false;

        while agent_o | agent_x != FILLED_BOARD {
            let best_move = get_best_move(agent_x, agent_o, turn);
            if turn {
                agent_x = agent_x | best_move;
            } else {
                agent_o = agent_o | best_move;
            }
            print_u16_as_board(agent_x, agent_o);
            turn = !turn;
        }
    }
}
