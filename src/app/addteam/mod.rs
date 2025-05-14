use iced::{widget::{button, container}, Length};

use crate::app::{Screen};

#[derive(Debug, Clone)]
pub enum AddTeamMessage {}

pub struct AddTeam;

impl AddTeam {
    pub fn new() -> Self {
        Self
    }
}

impl Screen for AddTeam {
    fn update(&mut self, _message: super::MessageDispatcher) -> Option<Box<dyn Screen>> {
        None
    }

    fn view(&self) -> iced::Element<super::MessageDispatcher> {
        container(
            button("Adicionar time"),
        )
        .center(Length::Fill)
        .into()
    }
}