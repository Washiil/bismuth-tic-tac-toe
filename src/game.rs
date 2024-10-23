use crate::agent::Agent;
use crate::board;

struct TicTacToe {
    board: u16,
    agent_x: u16,
    agent_o: u16,
    turn: Agent
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: 0,
            agent_x: 0,
            agent_o: 0,
            turn: Agent::X
        }
    }

    
}