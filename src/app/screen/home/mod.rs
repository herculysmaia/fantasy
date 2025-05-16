use iced::{widget::{button, container}, Length, Task};

use crate::app::screen::{MessageDispatcher, Screen, AddTeam, ScreenTaskReturn};

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
    fn update(&mut self, message: MessageDispatcher) -> ScreenTaskReturn {
        match message {
            MessageDispatcher::Home(msg) => {
                match msg {
                    HomeMessage::GoToAddTeam => (Some(Box::new(AddTeam::new())), Task::none()),
                }
            }
            _ => (None, Task::none()),
            
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