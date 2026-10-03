use std::path::Path;

use cucumber::{World as _, given, then, when};

use flashpoint_curator::{AppState, HaloFlashPointCurator, PlayerTurnState};

#[derive(Debug, Default, cucumber::World)]
struct World {
    app: HaloFlashPointCurator,
}

#[given(expr = "the field to add a player contains {word}")]
async fn add_player_field_contains(world: &mut World, name: String) {
    world.app.adding_player_name = name;
}

#[given(expr = "the app contains a player called {word}")]
async fn given_app_contains_player(world: &mut World, name: String) {
    add_player_field_contains(world, name).await;
    add_player_button_clicked(world).await;
}

#[when("the add player button is clicked")]
async fn add_player_button_clicked(world: &mut World) {
    world.app.handle_add_player_click();
}

#[when("the start game button is clicked")]
async fn start_game_button_clicked(world: &mut World) {
    world.app.handle_start_game_click();
}

#[when(expr = "the remove player button for {word} is clicked")]
async fn remove_button_clicked(world: &mut World, name: String) {
    let player = world.app.players.iter().find(|p| p.name == name);
    world
        .app
        .handle_remove_player_click(&player.expect("failed to find player").clone());
}

#[then(expr = "the app contains a player called {word}")]
async fn app_contains_player(world: &mut World, name: String) {
    let find_player = world.app.players.iter().find(|p| p.name == name);
    assert!(find_player.is_some());
}

#[then(expr = "the app does not contain a player called {word}")]
async fn app_doesnt_contains_player(world: &mut World, name: String) {
    let player_search = world.app.players.iter().find(|p| p.name == name);

    assert!(player_search.is_none());
}

#[then(expr = "the player {word} has zeroed stats")]
async fn player_is_zeroed(world: &mut World, name: String) {
    let player = world.app.players.iter().find(|p| p.name == name);

    let player = player.expect("Did not contain player");

    assert_eq!(player.kill_count, 0);
    assert_eq!(player.next_turn_order_number, 0);
    assert_eq!(player.current_turn_order_number, 0);
    assert_eq!(player.turn_state, PlayerTurnState::Playing);
    assert_eq!(player.played_turn_count, 0);
}

#[then(expr = "the player {word} has a turn order number but no kills")]
async fn player_has_turn_order(world: &mut World, name: String) {
    let player = world.app.players.iter().find(|p| p.name == name);

    let player = player.expect("Did not contain player");

    assert_ne!(player.current_turn_order_number, 0);
    assert_eq!(player.kill_count, 0);
}

#[then(expr = "the app is in the {word} state")]
async fn app_is_in_x_state(world: &mut World, state: String) {
    let app_state = match state.as_str() {
        "Configuring" => AppState::Configuring,
        "Massive-Slayer-Multiplayer" => AppState::MassiveSlayer,
        _ => panic!("unknown state!"),
    };

    assert_eq!(world.app.state, app_state);
}

#[tokio::main]
async fn main() {
    World::run(Path::new("./features/configuring.feature")).await;
}
