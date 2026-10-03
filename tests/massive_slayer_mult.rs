use std::path::Path;

use cucumber::{World as _, given};

mod common;
use common::World;


#[tokio::main]
async fn main() {
    World::run(Path::new("./features/massive_slayer_mult.feature")).await;
}