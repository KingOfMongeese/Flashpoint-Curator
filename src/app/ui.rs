//! Contains the mappings from UI to Application logic
use std::collections::VecDeque;

use super::{AppState, HaloFlashPointCurator};

use crate::app::{
    handle_add_kill_click, handle_player_pass_click, handle_remove_kill_click, handle_spartan_click_for, player::PlayerTurnState,
};
use egui::{Key, Slider};

impl eframe::App for HaloFlashPointCurator {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        match self.state {
            AppState::Configuring => self.configuring_ui(ui),
            AppState::MassiveSlayer => self.massive_slayer_mult_ui(ui),
        }

        powered_by_egui_and_eframe(ui);
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

impl HaloFlashPointCurator {
    fn configuring_ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Configuring Halo Flashpoint Curator");

            ui.horizontal(|ui| {
                let name_label = ui.label("Enter player name");
                ui.text_edit_singleline(&mut self.adding_player_name)
                    .labelled_by(name_label.id);

                if ui.button("Add Player").clicked() || ui.input(|i| i.key_pressed(Key::Enter)) {
                    self.handle_add_player_click();
                }
            });

            ui.label("Players");

            for player in self.players.clone() {
                ui.horizontal(|ui| {
                    ui.label(player.name.clone());
                    if ui.button("Remove").clicked() {
                        self.handle_remove_player_click(&player);
                    }
                });
            }
            ui.label("_____________________________");

            ui.label("Game Settings");
            ui.add(
                Slider::new(&mut self.battle_royal_kill_required, 2..=16)
                    .text("Kills to trigger Battle Royal mode"),
            );
            ui.label("_____________________________");

            if ui.button("Start Game").clicked() {
                self.handle_start_game_click();
            }
        });
    }

    fn massive_slayer_mult_ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Massive Slayer Multiplayer");
            ui.label(format!("Turn: {}", self.turn_count));

            // temp redirect to the spartans used for this frame
            let mut spartans_used = VecDeque::with_capacity(2);

            // check if a player has won
            if self.players.len() == 1 {
                ui.heading("Victory!!!");
                let player = &self.players[0];
                ui.label(format!(
                    "{}: total_kills: {}, Played {} turns",
                    player.name, player.kill_count, player.played_turn_count
                ));

                if ui.button("New Game").clicked() {
                    self.reset();
                }
            } else {
                for player in &mut self.players {
                    ui.horizontal(|ui| {
                        ui.label(format!("{} ", player.current_turn_order_number));
                        ui.label(player.name.clone());
                        ui.label(format!("Kills: {}", player.kill_count));

                        if ui.button("Remove Kill").clicked() {
                            handle_remove_kill_click(player);
                        }

                        if ui.button("Add Kill").clicked() {
                            handle_add_kill_click(player);
                        }

                        if player.turn_state == PlayerTurnState::Playing {
                            if ui.button("Pass").clicked() {
                                handle_player_pass_click(&mut self.turn_slot, player);
                            }
                        } else {
                            ui.label(format!("Passed {}", player.next_turn_order_number));
                            if ui.button("Spartan").clicked() {
                                handle_spartan_click_for(&mut spartans_used, player.id);
                            }
                        }
                    });
                }
            }

            for player_id in spartans_used {
                    let player = self.players.iter_mut().find(|p| p.id == player_id);
                    if let Some(player) = player {
                        player.spartans_used += 1;
                        self.handle_spartans_for.push_back(player_id);
                    }
                }


            if !self.handle_spartans_for.is_empty() {
                ui.label("_____________________________");
                ui.label("Spartan's used that will affect next turn order");

                for player in &self.players {
                    if player.spartans_used > 0 {
                        ui.label(format!("{}: {}", player.name, player.spartans_used));
                    }
                }
            }

            if !self.eliminated_players.is_empty() {
                ui.label("_____________________________");
                ui.label("Eliminated Players");
                for player in &self.eliminated_players {
                    ui.label(format!(
                        "{}: total_kills: {}, Played {} turns",
                        player.name, player.kill_count, player.played_turn_count
                    ));
                }
                ui.label("_____________________________");
            }

            // only display end turn button if all players have passed
            if self
                .players
                .iter()
                .find(|p| p.turn_state == PlayerTurnState::Playing)
                .is_none()
            {
                if ui.button("End Turn").clicked() {
                    self.handle_end_turn_click();
                }
            }
        });
    }
}
