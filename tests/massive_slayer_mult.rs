use std::path::Path;

use cucumber::{World as _, when, then, given};

mod common;
use common::World;
use flashpoint_curator::HaloFlashPointCurator;

#[given(expr = "{word}'s kills is {word}")]
async fn set_kills_to(world: &mut World, name: String, new_kills: usize) {
    let player = common::get_player_by_name_mut(world, &name).expect("failed to get player");
    player.kill_count = new_kills;
}

#[when(expr = "the add kill button for {word} is clicked")]
async fn player_kill_incremented(world: &mut World, name: String) {
    let player = common::get_player_by_name_mut(world, &name).expect("failed to get player");
    flashpoint_curator::app::handle_add_kill_click(player);
}

#[when(expr = "the pass button for {word} is clicked")]
async fn pass_button_clicked(world: &mut World, name: String) {
    let player = world.app.players.iter_mut().find(|p| p.name == name).expect("failed to find player");
    let turn_slot = &mut world.app.turn_slot;
    flashpoint_curator::app::handle_player_pass_click(turn_slot, player);
}

#[when(expr = "the next turn button is clicked")]
async fn next_turn(world: &mut World) {
    world.app.handle_end_turn_click();
}

#[when(expr = "the new game button is clicked")]
async fn reset(world: &mut World) {
    world.app.reset();
}

#[when(expr = "the remove kill button for {word} is clicked")]
async fn remove_kill_click(world: &mut World, name: String) {
    let player = common::get_player_by_name_mut(world, &name).expect("failed to get player");
    flashpoint_curator::app::handle_remove_kill_click(player);
}

#[then(expr = "{word}'s kills is {word}")]
async fn verify_kills_incremented(world: &mut World, name: String, expected_kills: usize) {
    let player = common::get_player_by_name(world, &name).expect("failed to get player");
    assert_eq!(player.kill_count, expected_kills);
}

#[then(expr = "{word}'s next turn number is {word}")]
async fn verify_next_turn_number(world: &mut World, name: String, expected_turn_number: usize) {
    let player = common::get_player_by_name(world, &name).expect("failed to get player");
    assert_eq!(player.next_turn_order_number, expected_turn_number);
    
}

#[then(expr = "{word}'s current turn number is {word}")]
async fn verify_current_turn_number(world: &mut World, name: String, expected_turn_number: usize) {
    let player = common::get_player_by_name(world, &name).expect("failed to get player");
    assert_eq!(player.current_turn_order_number, expected_turn_number);   
}

#[then(expr = "the app's data is set back to default")]
async fn verify_default_on_restart(world: &mut World) {
    let default_app = HaloFlashPointCurator::default(); 
    assert_eq!(world.app, default_app);
}

#[tokio::main]
async fn main() {
    World::run(Path::new("./features/massive_slayer_mult.feature")).await;
}
