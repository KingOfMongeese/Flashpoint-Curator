use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PlayerTurnState {
    Playing,
    Passed,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub kill_count: usize,
    pub current_turn_order_number: usize,
    pub next_turn_order_number: usize,
    pub id: usize,
    pub turn_state: PlayerTurnState,
    pub played_turn_count: usize,
}

impl Player {
    pub fn end_turn(&mut self) {
        self.current_turn_order_number = self.next_turn_order_number;
        self.next_turn_order_number = 0;
        self.turn_state = PlayerTurnState::Playing;
        self.played_turn_count += 1;
    }

    pub fn new(name: String, id: usize) -> Self {
        Self {
            name,
            kill_count: 0,
            current_turn_order_number: 0,
            next_turn_order_number: 0,
            id,
            turn_state: PlayerTurnState::Playing,
            played_turn_count: 0,
        }
    }
}
