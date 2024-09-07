pub mod messages;
mod state;

use crate::state::AppState;
use iced::{Sandbox, Settings};

fn main() -> iced::Result {
    AppState::run(Settings::default())
}
