pub mod app;

use app::App;

fn main() -> iced::Result {
    iced::application("Liga", App::update, App::view).theme(|_| app::theme()).run_with(App::new)
}