//! Contains the mappings from UI to Application logic

use super::{AppState, HaloFlashPointCurator, Player};

use crate::app::player::PlayerTurnState;
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
                            player.kill_count = player.kill_count.saturating_sub(1);
                        }

                        if ui.button("Add Kill").clicked() {
                            player.kill_count += 1;
                        }

                        if player.turn_state == PlayerTurnState::Playing {
                            if ui.button("Pass").clicked() {
                                player.next_turn_order_number = self.turn_slot;
                                self.turn_slot += 1;
                                player.turn_state = PlayerTurnState::Passed;
                            }
                        } else {
                            ui.label(format!("Passed {}", player.next_turn_order_number));
                        }
                    });
                }
            }

            if !self.eliminated_players.is_empty() {
                ui.label("_____________________________");
                ui.label("Elimanated Players");
                for player in &self.eliminated_players {
                    ui.label(format!(
                        "{}: total_kills: {}, Played {} turns",
                        player.name, player.kill_count, player.played_turn_count
                    ));
                }
                ui.label("_____________________________");
            }

            // only display end turn button if all playes have passed
            if self
                .players
                .iter()
                .find(|p| p.turn_state == PlayerTurnState::Playing)
                .is_none()
            {
                if ui.button("End Turn").clicked() {
                    // resfresh player turn slots
                    self.turn_slot = 1;
                    for player in &mut self.players {
                        player.end_turn();
                    }

                    // see if we are in battle royal
                    if self
                        .players
                        .iter()
                        .find(|p| p.kill_count >= self.battle_royal_kill_required)
                        .is_some()
                    {
                        let mut highest_kill_cnt = self.players[0].kill_count;
                        for player in &self.players {
                            if player.kill_count > highest_kill_cnt {
                                highest_kill_cnt = player.kill_count;
                            }
                        }

                        self.battle_royal_kill_required = highest_kill_cnt;

                        // get all the players we will elimnate
                        let elimated_players: Vec<Player> = self
                            .players
                            .iter()
                            .filter(|p| p.kill_count < self.battle_royal_kill_required)
                            .map(|p| p.clone())
                            .collect();

                        self.eliminated_players.extend(elimated_players);

                        // only keep players that meet the kill count
                        self.players
                            .retain(|p| p.kill_count >= self.battle_royal_kill_required);

                        // now the kill count will be 1 higher
                        self.battle_royal_kill_required += 1;

                        // reset the turn slots as we removed players
                        self.players.sort_by(|a, b| {
                            a.current_turn_order_number
                                .cmp(&b.current_turn_order_number)
                        });

                        // fix the turn slots number
                        self.players
                            .iter_mut()
                            .enumerate()
                            .for_each(|(corrected_turn_slot, p)| {
                                p.current_turn_order_number = corrected_turn_slot + 1
                            });
                    }

                    // sort the players by turn order number
                    self.players.sort_by(|a, b| {
                        a.current_turn_order_number
                            .cmp(&b.current_turn_order_number)
                    });

                    self.turn_count += 1;
                }
            }
        });
    }
}
