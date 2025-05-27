mod home;
mod addteam;
mod rodadas;
mod finance;
mod common;

pub use home::Home;
pub use addteam::AddTeam;
pub use finance::Finance;
pub use rodadas::Rodadas;

use common::theme as definir_tema;

use iced::Task;

#[derive(Debug, Clone)]
pub enum MessageDispatcher {
    Home(home::HomeMessage),
    AddTeam(addteam::AddTeamMessage),
    Rodadas(rodadas::RodadasMessage),
    Finance(finance::FinanceMessage),
}

pub type ScreenTaskReturn = (Option<Box<dyn Screen>>, Task<MessageDispatcher>);

pub trait Screen {
    fn update(&mut self, message: MessageDispatcher) -> ScreenTaskReturn;
    fn view(&self) -> iced::Element<MessageDispatcher>;
}

pub struct App {
    screen: Box<dyn Screen>,
}

impl App {
    pub fn new() -> (Self, Task<MessageDispatcher>) {
        (
            Self {
                screen: Box::new(Home::new()),
            },
            Task::none(),
        )
    }

    pub fn theme() -> iced::Theme {
        definir_tema()
    }

    pub fn update(&mut self, message: MessageDispatcher) -> Task<MessageDispatcher> {
        let (page, msg) = self.screen.update(message);
        
        if let Some(s) = page {
            self.screen = s;
        }
        
        msg
    }

    pub fn view(&self) -> iced::Element<MessageDispatcher> {
        self.screen.view()
    }
}