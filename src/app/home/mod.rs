use iced::{widget::{button, container}, Length};

use crate::app::{MessageDispatcher, Screen, AddTeam};

#[derive(Debug, Clone)]
pub enum HomeMessage {
    GoToAddTeam,
}

pub struct Home;

impl Home {
    pub fn new() -> Self {
        Self
    }
}

impl Screen for Home {
    fn update(&mut self, message: super::MessageDispatcher) -> Option<Box<dyn Screen>> {
        match message {
            MessageDispatcher::Home(msg) => {
                match msg {
                    HomeMessage::GoToAddTeam => Some(Box::new(AddTeam::new())),
                }
            }
            _ => None,
            
        }
    }

    fn view(&self) -> iced::Element<super::MessageDispatcher> {
        container(
            button("Adicionar time")
                .on_press(MessageDispatcher::Home(HomeMessage::GoToAddTeam)),
        )
        .center(Length::Fill)
        .into()
    }
}