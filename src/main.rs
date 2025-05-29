mod app;

pub use app::App;

fn main() -> iced::Result {
    let _ = app::criar_banco();
    iced::application("Liga Melhor da Rodada - Ano 9", App::update, App::view).theme(|_| App::theme()).run_with(App::new)
}