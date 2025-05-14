pub mod style;
pub mod app;

use style::theme;
use app::App;

fn main() -> iced::Result {
    iced::application("Liga", App::update, App::view).theme(|_| theme()).run_with(App::new)
}