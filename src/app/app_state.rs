use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum AppState {
    /// Get the names of all the players and set any config options
    Configuring,

    /// Run the main help for massive slayer
    MassiveSlayer,
}
