use iced::Task;

use crate::app::{api::{buscar_rodada_atual, obter_pontuacoes, ApiError}, obter_times, Time};

use super::{MessageDispatcher, Screen, ScreenTaskReturn};

use iced::widget::{column, text, row};

#[derive(Debug, Clone)]
pub enum RodadasMessage {
    DefinirRodadaAtual(u32),
    ExibirPontuacao(Result<Vec<Time>, ApiError>)
}

pub struct Rodadas{
    atual: u32,
    lista_de_times: Vec<Time>,
}

fn message_proc(msg: RodadasMessage) -> MessageDispatcher {
    MessageDispatcher::Rodadas(msg)
}

impl Rodadas {
    pub fn new() -> Self {
        Self { atual: 0, lista_de_times: Vec::new() }
    }

    pub fn task_inicial() -> iced::Task<MessageDispatcher> {
        Task::perform(async { buscar_rodada_atual().await }, |atual| message_proc(RodadasMessage::DefinirRodadaAtual(atual.unwrap_or(0))))
    }
}

impl Screen for Rodadas {
    fn update(&mut self, message: MessageDispatcher) -> ScreenTaskReturn {
        use RodadasMessage::*;

        let mut retorno: ScreenTaskReturn = (None, Task::none());

        match message {
            MessageDispatcher::Rodadas(DefinirRodadaAtual(atual)) => {
                self.atual = atual - 1;

                let lista_de_times = obter_times();

                if self.atual != 0 {
                    retorno = (None, buscar_pontuacao_atual(self.atual, lista_de_times.clone()));
                }
            },
            MessageDispatcher::Rodadas(ExibirPontuacao(lista_de_times)) => {
                match lista_de_times {
                    Ok(times) => self.lista_de_times = times,
                    Err(e) => {
                        println!("Erro ao obter pontuações: {:?}", e);
                        retorno = (None, Task::none());
                    }
                    
                }
            }
                
            _ => println!("Mensagem de RodadasMessage não mapeada: {:?}", message),
        }

        retorno
    }

    fn view(&self) -> iced::Element<MessageDispatcher> {
        iced::widget::container(exibir_pontuacao(&self.lista_de_times, self.atual))
            .into()
    }
}

fn buscar_pontuacao_atual(rodada: u32, lista_de_times: Vec<Time>) -> Task<MessageDispatcher> {
    Task::perform(async move { obter_pontuacoes(rodada, lista_de_times).await }, |result| message_proc(RodadasMessage::ExibirPontuacao(result)))
}

fn exibir_pontuacao(lista_de_times: &Vec<Time>, rodada: u32) -> iced::Element<MessageDispatcher> {

    let mut col = column![];

    if lista_de_times.is_empty() {
        col = col.push(text("Nenhum time encontrado na rodada atual."));
    } else {
        for time in lista_de_times {
            if let Some(pontuacao) = time.pontos.iter().find(|p| p.rodada == rodada) {
                col = col.push(
                    row![
                        text(format!("Time: {} | Pontuação: {}", time.nome_do_time, pontuacao.pontos)),
                    ]
                );
            }
        }
    }

    col.padding(10).into()
}
