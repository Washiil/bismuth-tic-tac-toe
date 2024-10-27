use std::future;

pub mod agent;
pub mod board;
pub mod game;
pub mod error;

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

fn minimax(
    agent_x: u16,
    agent_o: u16,
    mut alpha: i32,
    mut beta: i32,
    depth: i32,
    maximizing_player: bool,
) -> (i32, u16) {
    // Check for terminal states
    let score = board::evaluate_board(agent_x, agent_o);
    match score.cmp(&0) {
        std::cmp::Ordering::Greater => return (score - depth, 0),
        std::cmp::Ordering::Less => return (score + depth, 0),
        std::cmp::Ordering::Equal => (),
    }

    // Check for leaf nodes
    if depth == 0 || board::is_game_draw(agent_x | agent_o) {
        return (0, 0);
    }

    let possible_moves = board::generate_moves(agent_x | agent_o);

    if maximizing_player {
        let mut best_score = i32::MIN;
        let mut best_move = 0;

        for pos in possible_moves {
            // Make the move and recurse
            let (score, _) = minimax(agent_x | pos, agent_o, alpha, beta, depth - 1, false);

            if score > best_score {
                best_score = score;
                best_move = pos;
            }

            alpha = alpha.max(best_score);
            if beta <= alpha {
                break; // Beta cutoff
            }
        }

        (best_score, best_move)
    } else {
        let mut best_score = i32::MAX;
        let mut best_move = 0;

        for pos in possible_moves {
            // Make the move and recurse
            let (score, _) = minimax(agent_x, agent_o | pos, alpha, beta, depth - 1, true);

            if score < best_score {
                // Changed from > to < for minimizing player
                best_score = score;
                best_move = pos;
            }

            beta = beta.min(best_score); // Changed from alpha.max to beta.min
            if beta <= alpha {
                break; // Alpha cutoff
            }
        }

        (best_score, best_move)
    }
}

pub fn get_best_move(agent_x: u16, agent_o: u16, is_x_turn: bool) -> u16 {
    let (_, best_move) = minimax(agent_x, agent_o, i32::MIN, i32::MAX, 9, is_x_turn);
    best_move
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
