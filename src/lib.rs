use std::future;

pub mod board;
pub mod game;
pub mod agent;

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

/// Evaluates the position for a win
fn evaluate_position(agent_x: u16, agent_o: u16) -> i32 {
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

/// Generates all possible tic-tac-toe moves by iterating over each of the bits and returning bits that dont have a space.
pub fn generate_moves(board: u16) -> Vec<u16> {
    (0..9)
        .map(|i| 1u16 << i)
        .filter(|mask| board & mask == 0)
        .collect()
}

pub fn alpha_beta(agent_x: u16, agent_o: u16, alpha: &mut i32, beta: &mut i32, depth: i32, maximizing_player: bool) -> (i32, u16) {
    let score = evaluate_position(agent_x, agent_o);

    match score.cmp(&0) {
        std::cmp::Ordering::Greater => {return (score - depth, 0)}
        std::cmp::Ordering::Equal => (),
        std::cmp::Ordering::Less => {return (score + depth, 0)}
    }

    if depth == 0 || is_game_draw(agent_x | agent_o) {
        return (0, 0) // Game was a draw and we ran out of depth
    }

    let possible_moves = generate_moves(agent_x | agent_o);

    if maximizing_player {
        let mut best_score = i32::MIN;
        let mut best_move = 0;

        for pos in possible_moves {
            let future_eval = evaluate_position(agent_x | pos, agent_o);
            if future_eval > best_score {
                best_score = future_eval;
                best_move = pos;
            }

            alpha = alpha.max(&mut best_score);
            if beta <= alpha {
                break;
            }
        }
        return (best_score, best_move)
    }
    else {
        let mut best_score = i32::MAX;
        let mut best_move = 0;

        for pos in possible_moves {
            let future_eval = evaluate_position(agent_x, agent_o | pos);
            if future_eval > best_score {
                best_score = future_eval;
                best_move = pos;
            }

            beta = alpha.max(&mut best_score);
            if beta <= alpha {
                break;
            }
        }
        return (best_score, best_move)
    }

    (0, 0)
}

pub fn minimax(agent_x: u16, agent_o: u16, depth: i32, maximizing_player: bool) -> (i32, u16) {
    let score = evaluate_position(agent_x, agent_o);
    
    // Check if the game is in an terinary state
    if score != 0 || depth == 0 || is_game_draw(agent_x | agent_o) {
        return (score, 0);
    }

    if maximizing_player {
        let mut best_value = i32::MIN;
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
        let mut best_value = i32::MAX;
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
