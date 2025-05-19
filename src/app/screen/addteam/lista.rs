use iced::{widget::{column, row, button, text}, Element, Task};

use crate::app::{MessageDispatcher, Screen, ScreenTaskReturn, Time};

use super::AddTeamMessage;
use super::super::common::WhiteFrame;

pub struct Lista {
    resultados: Vec<Time>,
}

#[derive(Debug, Clone)]
pub enum ListaMessage {
    SetResultados(Vec<Time>),
    Add(u32),
}

fn message_proc(msg: ListaMessage) -> MessageDispatcher {
    MessageDispatcher::AddTeam(AddTeamMessage::Lista(msg))
}

fn listar_time(lista: &Vec<Time>) -> Element<MessageDispatcher> {
    let col = {
        if lista.is_empty() {
            column![text("Nenhum time encontrado")]
        } else {
            lista.iter().fold(
                column![], |col, time| { 
                    col.push(
                        row![
                            text(format!("Time: {} | Dono: {}", time.nome_do_time, time.nome_do_dono)),
                            button("Adicionar").on_press(message_proc(ListaMessage::Add(time.id)))]
                    )
                }
            )
        }
    };

    col.padding(5).spacing(10).into()
}

impl Lista {
    pub fn new() -> Self {
        Self {
            resultados: Vec::new(),
        }
    }
    pub fn set_resultados(&mut self, resultados: Vec<Time>) {
        self.resultados = resultados;
    }
}

impl Screen for Lista {
    fn update(&mut self, message: MessageDispatcher) -> ScreenTaskReturn {
        match message {
            MessageDispatcher::AddTeam(AddTeamMessage::Lista(ListaMessage::SetResultados(resultado))) => {
                self.set_resultados(resultado);
            }
            MessageDispatcher::AddTeam(AddTeamMessage::Lista(ListaMessage::Add(id))) => {
                println!("{id}");
            }
            _ => (),
        }

        (None, Task::none())
    }

    fn view(&self) -> Element<MessageDispatcher> {
        WhiteFrame::new( listar_time(&self.resultados)).into()
    }
}
