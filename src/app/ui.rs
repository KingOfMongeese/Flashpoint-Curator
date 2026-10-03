//! Contains the mappings from UI to Application logic

use super::{HaloFlashPointCurator, AppState, Player};

use egui::{Key, Slider};
use rand::seq::SliceRandom;


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

                if (ui.button("Add Player").clicked() || ui.input(|i| i.key_pressed(Key::Enter)))
                    && !self.adding_player_name.is_empty()
                {
                    self.players.push(Player::new(
                        self.adding_player_name.clone(),
                        self.id_counter,
                    ));
                    self.adding_player_name.clear();
                    self.id_counter += 1;
                }
            });

            ui.label("Players");

            let mut player_to_remove = None;
            for player in &self.players {
                ui.horizontal(|ui| {
                    ui.label(player.name.clone());
                    if ui.button("Remove").clicked() {
                        player_to_remove = self
                            .players
                            .iter()
                            .enumerate()
                            .find(|(_, p)| p.id == player.id)
                            .map(|(idx, _)| idx);
                    }
                });
            }
            ui.label("_____________________________");

            if let Some(player_idx) = player_to_remove {
                self.players.remove(player_idx);
            }

            ui.label("Game Settings");
            ui.add(
                Slider::new(&mut self.battle_royal_kill_required, 2..=16)
                    .text("Kills to trigger Battle Royal mode"),
            );
            ui.label("_____________________________");

            if ui.button("Start Game").clicked() {
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
        });
    }
}
