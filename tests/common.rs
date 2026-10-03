use cucumber::{given, then, when};

use flashpoint_curator::{HaloFlashPointCurator, AppState};

#[derive(Debug, Default, cucumber::World)]
pub struct World {
    pub app: HaloFlashPointCurator,
}

#[given(expr = "the field to add a player contains {word}")]
pub async fn add_player_field_contains(world: &mut World, name: String) {
    world.app.adding_player_name = name;
}

#[given(expr = "the app contains a player called {word}")]
pub async fn given_app_contains_player(world: &mut World, name: String) {
    add_player_field_contains(world, name).await;
    add_player_button_clicked(world).await;
}

#[when("the add player button is clicked")]
pub async fn add_player_button_clicked(world: &mut World) {
    world.app.handle_add_player_click();
}

#[when("the start game button is clicked")]
pub async fn start_game_button_clicked(world: &mut World) {
    world.app.handle_start_game_click();
}

#[then(expr = "the app is in the {word} state")]
pub async fn app_is_in_x_state(world: &mut World, state: String) {
    let app_state = match state.as_str() {
        "Configuring" => AppState::Configuring,
        "Massive-Slayer-Multiplayer" => AppState::MassiveSlayer,
        _ => panic!("unknown state!"),
    };

    assert_eq!(world.app.state, app_state);
}