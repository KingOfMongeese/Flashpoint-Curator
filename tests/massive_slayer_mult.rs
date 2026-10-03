use std::path::Path;

use cucumber::{World as _, when, then, given};

mod common;
use common::World;

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

#[tokio::main]
async fn main() {
    World::run(Path::new("./features/massive_slayer_mult.feature")).await;
}
