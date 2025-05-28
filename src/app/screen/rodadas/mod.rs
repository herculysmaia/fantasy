use iced::Task;

use crate::app::db::obter_ultima_rodada_salva_no_banco;
use crate::app::{api::{buscar_rodada_atual, obter_pontuacoes, ApiError}, obter_times, Time};

use super::{MessageDispatcher, Screen, ScreenTaskReturn};

use iced::widget::{column, text, row, button};

#[derive(Debug, Clone)]
pub enum RodadasMessage {
    DefinirRodadaAtual(u32),
    ExibirPontuacao(Result<Vec<Time>, ApiError>),
    SalvarPontuacao(Vec<Time>, u32),
    PontuacaoSalva,
}

pub struct Rodadas{
    atual: u32,
    salvo_no_banco: bool,
    lista_de_times: Vec<Time>,
}

fn message_proc(msg: RodadasMessage) -> MessageDispatcher {
    MessageDispatcher::Rodadas(msg)
}

impl Rodadas {
    pub fn new() -> Self {
        Self { atual: 0, salvo_no_banco: false, lista_de_times: Vec::new() }
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
                let rodada_do_banco = obter_ultima_rodada_salva_no_banco();

                if rodada_do_banco < atual - 1 {
                    self.atual = rodada_do_banco + 1;
                } else {
                    self.atual = atual - 1;
                    self.salvo_no_banco = true;
                }

                if self.atual < 1 {
                    println!("A rodada atual é menor que 1");
                    self.atual = 1;
                }

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
            MessageDispatcher::Rodadas(SalvarPontuacao(lista_de_times, rodada)) => {
                if !lista_de_times.is_empty() {
                    let task = Task::perform(async move { Time::salvar_pontuacoes(lista_de_times, rodada).await }, |_| message_proc(RodadasMessage::PontuacaoSalva));
                    retorno = (None, task);
                    self.salvo_no_banco = true;
                } else {
                    println!("Nenhum time para salvar pontuação.");
                }
            },
                
            _ => println!("Mensagem de RodadasMessage não mapeada: {:?}", message),
        }

        retorno
    }

    fn view(&self) -> iced::Element<MessageDispatcher> {
        iced::widget::container(
            column![
                {
                    let botao_salvar = button(text("Salvar Pontuação"));

                    if self.salvo_no_banco || self.lista_de_times.is_empty() {
                        botao_salvar
                    } else {
                        botao_salvar.on_press(message_proc(RodadasMessage::SalvarPontuacao(self.lista_de_times.clone(), self.atual)))
                    }
                },
                exibir_pontuacao(&self.lista_de_times, self.atual)
            ])
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
