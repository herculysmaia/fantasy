use std::time::Duration;

use iced::futures::channel::oneshot;
use iced::widget::{row, text_input};
use iced::Task;

use tokio::time::sleep;

use crate::app::screen::{addteam::AddTeamMessage, MessageDispatcher, Screen, ScreenTaskReturn};
use crate::app::api::buscar_time;

#[derive(Debug, Clone)]
pub enum BuscaMessage {
    Busca(String),
    TimeEnd(String),
    TimeNotEnd,
}

pub struct Busca {
    termo: String,
    time_search: Option<oneshot::Sender<()>>,
}

fn message_proc(msg: BuscaMessage) -> MessageDispatcher {
    MessageDispatcher::AddTeam(AddTeamMessage::Busca(msg))
}

impl Busca {
    pub fn new() -> Self {
        Self {
            termo: String::new(),
            time_search: None,
        }
    }
}

impl Screen for Busca {
    fn update(&mut self, message: MessageDispatcher) -> ScreenTaskReturn {

        match message {
            MessageDispatcher::AddTeam(AddTeamMessage::Busca(BuscaMessage::Busca(s))) => {
                self.termo = s.clone();

                if let Some(sender) = self.time_search.take() {
                    let _ = sender.send(());
                }

                let (tx, rx) = oneshot::channel();

                self.time_search = Some(tx);

                let task = Task::perform(async move {
                    tokio::select! {
                        _ = sleep(Duration::from_secs(10)) => message_proc(BuscaMessage::TimeEnd(s)),
                        _ = rx => message_proc(BuscaMessage::TimeNotEnd),
                    }
                }, |msg| msg);

                (None, task)
            }
            MessageDispatcher::AddTeam(AddTeamMessage::Busca(BuscaMessage::TimeEnd(s))) => {
                let r = buscar_time(s);
                match r {
                    Ok(url) => {
                        println!("URL: {:?}", url);
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
                (None, Task::none())
            }
            MessageDispatcher::AddTeam(AddTeamMessage::Busca(BuscaMessage::TimeNotEnd)) => {
                (None, Task::none())
            }
            MessageDispatcher::Home(_) => (None, Task::none()),
            
        }
    }

    fn view(&self) -> iced::Element<MessageDispatcher> {
        row![
            text_input("Nome do time", &self.termo).on_input(|s| {
                MessageDispatcher::AddTeam(
                    AddTeamMessage::Busca(BuscaMessage::Busca(s))
                )
            })
        ].into()
    }
}
