mod busca;
mod lista;

use iced::Task;
use iced::widget::column;

use crate::app::{MessageDispatcher, Screen, ScreenTaskReturn};

use busca::{Busca, BuscaMessage};
use lista::{Lista, ListaMessage};

#[derive(Debug, Clone)]
pub enum AddTeamMessage {
    Busca(BuscaMessage),
    Lista(ListaMessage),
}

pub struct AddTeam{
    componete_busca: Busca,
    componete_lista: Lista,
}

fn message_proc(msg: AddTeamMessage) -> MessageDispatcher {
    MessageDispatcher::AddTeam(msg)
}

impl AddTeam {
    pub fn new() -> Self {
        Self {
            componete_busca: Busca::new(),
            componete_lista: Lista::new(),
        }
    }
}

impl Screen for AddTeam {
    fn update(&mut self, message: MessageDispatcher ) -> ScreenTaskReturn {
        match message {
            MessageDispatcher::AddTeam(AddTeamMessage::Busca(msg)) => {
                self.componete_busca.update(message_proc(AddTeamMessage::Busca(msg)))
            }
            MessageDispatcher::AddTeam(AddTeamMessage::Lista(msg)) => {
                self.componete_lista.update(message_proc(AddTeamMessage::Lista(msg)))
            }
            _ => (None, Task::none()),
        }
    }

    fn view(&self) -> iced::Element<MessageDispatcher> {
        column![
            self.componete_busca.view(),
            self.componete_lista.view()
        ].padding(10).spacing(10).into()
    }
}