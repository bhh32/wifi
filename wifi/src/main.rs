use cmd_lib::state::AppState;
use iced::{Sandbox, Settings};

fn main() -> iced::Result {
    AppState::run(Settings::default())
}
