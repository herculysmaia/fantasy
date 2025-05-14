mod home;
mod addteam;

pub use home::Home;
pub use addteam::AddTeam;
use iced::Task;

#[derive(Debug, Clone)]
pub enum MessageDispatcher {
    Home(home::HomeMessage),
    AddTeam(addteam::AddTeamMessage),
}

pub trait Screen {
    fn update(&mut self, message: MessageDispatcher) -> Option<Box<dyn Screen>>;
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

    pub fn update(&mut self, message: MessageDispatcher) {
        let page = self.screen.update(message);
        if let Some(s) = page {
            self.screen = s;
        }
    }

    pub fn view(&self) -> iced::Element<MessageDispatcher> {
        self.screen.view()
    }
}