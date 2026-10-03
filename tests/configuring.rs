use std::path::Path;

use cucumber::{World as _, then, when};

use flashpoint_curator::PlayerTurnState;

mod common;
use common::World;

#[when(expr = "the remove player button for {word} is clicked")]
async fn remove_button_clicked(world: &mut World, name: String) {
    let player = common::get_player_by_name(world, &name);
    world
        .app
        .handle_remove_player_click(&player.expect("failed to find player").clone());
}

#[then(expr = "the player {word} has zeroed stats")]
async fn player_is_zeroed(world: &mut World, name: String) {
    let player = common::get_player_by_name(world, &name);

    let player = player.expect("Did not contain player");

    assert_eq!(player.kill_count, 0);
    assert_eq!(player.next_turn_order_number, 0);
    assert_eq!(player.current_turn_order_number, 0);
    assert_eq!(player.turn_state, PlayerTurnState::Playing);
    assert_eq!(player.played_turn_count, 0);
}

#[then(expr = "the player {word} has a turn order number but no kills")]
async fn player_has_turn_order(world: &mut World, name: String) {
    let player = common::get_player_by_name(world, &name);

    let player = player.expect("Did not contain player");

    assert_ne!(player.current_turn_order_number, 0);
    assert_eq!(player.kill_count, 0);
}

#[tokio::main]
async fn main() {
    World::run(Path::new("./features/configuring.feature")).await;
}
