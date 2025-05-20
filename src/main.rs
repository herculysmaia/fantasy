mod app;

pub use app::App;

fn main() -> iced::Result {
    let _ = app::criar_banco();
    iced::application("Liga", App::update, App::view).theme(|_| App::theme()).run_with(App::new)
}