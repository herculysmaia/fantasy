use iced::{futures::channel::oneshot, widget::{row, text_input}, Element, Task};
use tokio::time::{sleep, Duration};

use crate::app::{buscar_time, screen::addteam::lista::ListaMessage, MessageDispatcher, Screen, ScreenTaskReturn, Time};

use super::AddTeamMessage;

pub struct Busca {
    query: String,
    temporizador: Option<oneshot::Sender<()>>,
}

#[derive(Debug, Clone)]
pub enum BuscaMessage {
    EntradaDeTexto(String),
    TimeReset,
}

fn message_proc(msg: BuscaMessage) -> MessageDispatcher {
    MessageDispatcher::AddTeam(AddTeamMessage::Busca(msg))
}

async fn buscar_time_na_api(query: String) -> Vec<Time>{
    let resposta = buscar_time(query).await;
    match resposta {
        Ok(resposta) => {
            return resposta;
        }
        Err(_) => {
            println!("Erro ao buscar o time");
            return vec![];
        }  
    }
}

impl Busca {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            temporizador: None,
        }
    }
}

impl Screen for Busca {
    fn update(&mut self, message: MessageDispatcher) -> ScreenTaskReturn {
        match message {
            MessageDispatcher::AddTeam(AddTeamMessage::Busca(BuscaMessage::EntradaDeTexto(s))) => {
                self.query = s.clone();

                if let Some(sender) = self.temporizador.take() {
                    let _ = sender.send(());
                }

                let (tx, rx) = oneshot::channel();

                self.temporizador = Some(tx);

                let task = Task::perform(async move {
                    tokio::select! {
                        _ = sleep(Duration::from_secs(1)) => MessageDispatcher::AddTeam(AddTeamMessage::Lista(ListaMessage::SetResultados(buscar_time_na_api(s).await))),
                        _ = rx => message_proc(BuscaMessage::TimeReset),
                    }
                }, |msg| msg);

                (None, task)
            }
            _ => (None, Task::none()),
        }
    }

    fn view(&self) -> Element<MessageDispatcher> {
        row![
            text_input("Nome do time", &self.query)
                .on_input(|s| message_proc(BuscaMessage::EntradaDeTexto(s))),
        ].into()
    }
}