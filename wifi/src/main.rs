pub mod messages;
mod state;

use crate::state::AppState;

fn main() -> cosmic::iced::Result {
    let settings = cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(600.0)
            .min_height(400.0),
    );

    cosmic::app::run::<AppState>(settings, ())
}
