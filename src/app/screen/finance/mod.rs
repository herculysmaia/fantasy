mod inicio;
mod time;

pub use inicio::{InicioFinance, InicioFinanceMessage};
pub use time::{TimeData, TimeDataMessage};

use crate::app::Screen;

use super::{MessageDispatcher, ScreenTaskReturn};

#[derive(Debug, Clone)]
pub enum FinanceMessage {
    Inicio(InicioFinanceMessage),
    TimeData(TimeDataMessage),
}

pub struct Finance {
    screen: Box<dyn Screen>,
}

impl Finance {
    pub fn new() -> Self {
        Self {
            screen: Box::new(InicioFinance::new()),
        }
    }
}

impl Screen for Finance {
    fn update(&mut self, message: MessageDispatcher) -> ScreenTaskReturn {
        let (page, msg) = self.screen.update(message);
        
        if let Some(s) = page {
            self.screen = s;
        }

        (None, msg)
    }

    fn view(&self) -> iced::Element<MessageDispatcher> {
        self.screen.view()
    }
}