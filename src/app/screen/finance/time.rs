use iced::{widget::{column}, Element, Task};

use crate::app::{MessageDispatcher, Screen, ScreenTaskReturn, Time};

use super::FinanceMessage;

pub struct TimeData;

#[derive(Debug, Clone)]
pub enum TimeDataMessage {
    Save(Time),
}

impl TimeData {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for TimeData {
    fn update(&mut self, message: MessageDispatcher) -> ScreenTaskReturn {
        use FinanceMessage::TimeData;
        use TimeDataMessage::*;

        match message {
            MessageDispatcher::Finance(TimeData(Save(time))) => {
                println!("{}", time.nome_do_time);

                (None, Task::none())
            }
            _ => (None, Task::none()),
        }
    }

    fn view(&self) -> Element<MessageDispatcher> {
        column![
        ].spacing(10).into()
    }
}
