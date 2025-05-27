use iced::{widget::{button, column, container, text}, Alignment, Length, Task};

use crate::app::screen::{MessageDispatcher, Screen, AddTeam, Finance, Rodadas, ScreenTaskReturn};


#[derive(Debug, Clone)]
pub enum HomeMessage {
    GoToAddTeam,
    GoToRodadas,
    GoToFinance,
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
                    HomeMessage::GoToRodadas => (Some(Box::new(Rodadas::new())), Rodadas::task_inicial()),
                    HomeMessage::GoToFinance => (Some(Box::new(Finance::new())), Task::none())
                }
            }
            _ => (None, Task::none()),
            
        }
    }

    fn view(&self) -> iced::Element<super::MessageDispatcher> {
        container(
            column![
                button(text("Adicionar time").width(Length::Fill).align_x(Alignment::Center)).on_press(MessageDispatcher::Home(HomeMessage::GoToAddTeam)).width(150),
                button(text("Rodadas").width(Length::Fill).align_x(Alignment::Center)).on_press(MessageDispatcher::Home(HomeMessage::GoToRodadas)).width(150),
                button(text("Financeiro").width(Length::Fill).align_x(Alignment::Center)).on_press(MessageDispatcher::Home(HomeMessage::GoToFinance)).width(150),
            ].align_x(Alignment::Center).spacing(10)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill) 
        .center_y(Length::Fill) 
.into()
    }
}