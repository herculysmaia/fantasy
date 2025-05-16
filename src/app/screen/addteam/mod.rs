mod busca;

use busca::Busca;

use crate::app::screen::{MessageDispatcher, Screen, ScreenTaskReturn};

use iced::{widget::container, Length};


#[derive(Debug, Clone)]
pub enum AddTeamMessage {
    Busca(busca::BuscaMessage),
}

pub struct AddTeam{
    campo_de_busca: Box<dyn Screen>,
}

impl AddTeam {
    pub fn new() -> Self {
        Self {
            campo_de_busca: Box::new(Busca::new()),
        }
    }
}

impl Screen for AddTeam {
    fn update(&mut self, message: MessageDispatcher ) -> ScreenTaskReturn {
        self.campo_de_busca.update(message)
    }

    fn view(&self) -> iced::Element<MessageDispatcher> {
        container(
           self.campo_de_busca.view(),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}