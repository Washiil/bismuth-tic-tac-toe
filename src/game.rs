use crate::agent::Agent;
use crate::board;
use crate::error::{TicTacToeError};

struct TicTacToe {
    board: u16,
    agent_x: u16,
    agent_o: u16,
    turn: Agent,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: 0b_000_000_000,
            agent_x: 0b_000_000_000,
            agent_o: 0b_000_000_0000,
            turn: Agent::X,
        }
    }

    pub fn make_move(&mut self, cell: u16) -> Result<u16, TicTacToeError> {
        match self.turn {
            Agent::X => todo!(),
            Agent::O => todo!(),
        }
    }
}