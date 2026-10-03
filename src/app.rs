mod app_state;
mod player;
mod ui;

pub use app_state::AppState;
pub use player::{Player, PlayerTurnState};
use serde::{Deserialize, Serialize};

use rand::seq::SliceRandom;

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct HaloFlashPointCurator {
    pub state: AppState,
    pub players: Vec<Player>,
    pub eliminated_players: Vec<Player>,
    pub adding_player_name: String,
    pub id_counter: usize,
    pub turn_slot: usize,
    pub battle_royal_kill_required: usize,
    pub turn_count: usize,
}

impl Default for HaloFlashPointCurator {
    fn default() -> Self {
        Self {
            state: AppState::Configuring,
            players: Vec::with_capacity(16),
            eliminated_players: Vec::with_capacity(16),
            adding_player_name: String::with_capacity(32),
            id_counter: 1,
            turn_slot: 1,
            battle_royal_kill_required: 4,
            turn_count: 1,
        }
    }
}

impl HaloFlashPointCurator {
    /// Called once before the first frame.
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl HaloFlashPointCurator {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn handle_add_player_click(&mut self) {
        // return early if the player name is empty, dont add empty players
        if self.adding_player_name.is_empty() {
            return;
        }

        self.players.push(Player::new(
            self.adding_player_name.clone(),
            self.id_counter,
        ));
        self.adding_player_name.clear();
        self.id_counter += 1;
    }

    pub fn handle_remove_player_click(&mut self, player: &Player) {
        let player_to_remove = self
            .players
            .iter()
            .enumerate()
            .find(|(_, p)| p.id == player.id)
            .map(|(idx, _)| idx);

        if let Some(player_idx) = player_to_remove {
            self.players.remove(player_idx);
        }
    }

    pub fn handle_start_game_click(&mut self) {
        self.state = AppState::MassiveSlayer;

        let mut turn_slots = Vec::with_capacity(self.players.len());
        for x in 1..=self.players.len() {
            turn_slots.push(x);
        }

        let mut rng = rand::rng();
        turn_slots.shuffle(&mut rng);
        for player in &mut self.players {
            player.current_turn_order_number = turn_slots.pop().unwrap();
        }

        self.players.sort_by(|a, b| {
            a.current_turn_order_number
                .cmp(&b.current_turn_order_number)
        });
    }
}
