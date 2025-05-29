use iced::{Alignment, Font, Task};

use crate::app::db::obter_ultima_rodada_salva_no_banco;
use crate::app::{api::{buscar_rodada_atual, obter_pontuacoes, ApiError}, obter_times, Time};

use super::common::WhiteFrame;
use super::{MessageDispatcher, Screen, ScreenTaskReturn};

use iced::widget::{button, column, row, text, Space};
use iced::{Length, Color};

#[derive(Debug, Clone)]
pub enum RodadasMessage {
    DefinirRodadaAtual(u32),
    ExibirPontuacao(Result<Vec<Time>, ApiError>),
    SalvarPontuacao(Vec<Time>, u32),
    PontuacaoSalva,
    AvancarRodada,
    RetrocederRodada,
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

                let precisa_buscar = lista_de_times.iter().any(|t| !t.pontos.iter().any(|p| p.rodada == self.atual));

                if self.atual != 0 && precisa_buscar {
                    retorno = (None, buscar_pontuacao_atual(self.atual, lista_de_times.clone()));
                } else {
                    self.lista_de_times = lista_de_times;
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
            MessageDispatcher::Rodadas(AvancarRodada) => {
                self.atual += 1;
                if self.atual >= 38 {
                    self.atual = 38
                }
            }
            MessageDispatcher::Rodadas(RetrocederRodada) => {
                self.atual -= 1;
                if self.atual <= 0 {
                    self.atual = 1
                }
            }
                
            _ => println!("Mensagem de RodadasMessage não mapeada: {:?}", message),
        }

        retorno
    }

    fn view(&self) -> iced::Element<MessageDispatcher> {
        iced::widget::container(
            column![
                row![
                    Space::with_width(Length::Fill),
                    button(text("<")).on_press(message_proc(RodadasMessage::RetrocederRodada)),
                {
                    let botao_salvar = button(text(format!("Rodada {}", self.atual)));

                    if self.salvo_no_banco || self.lista_de_times.is_empty() {
                        botao_salvar
                    } else {
                        botao_salvar.on_press(message_proc(RodadasMessage::SalvarPontuacao(self.lista_de_times.clone(), self.atual)))
                    }
                },
                    button(text(">")).on_press(message_proc(RodadasMessage::AvancarRodada)),
                    Space::with_width(Length::Fill),
                ]
                .width(Length::Fill)
                .spacing(10),
                exibir_pontuacao(&self.lista_de_times, self.atual)
            ].spacing(10))
            .padding(10)
            .into()
    }
}

fn buscar_pontuacao_atual(rodada: u32, lista_de_times: Vec<Time>) -> Task<MessageDispatcher> {
    Task::perform(async move { obter_pontuacoes(rodada, lista_de_times).await }, |result| message_proc(RodadasMessage::ExibirPontuacao(result)))
}

fn exibir_pontuacao(lista_de_times: &Vec<Time>, rodada: u32) -> iced::Element<MessageDispatcher> {
    let segoe_ui = Font::with_name("Segoe UI");
    let segoe_ui_bold = Font {
        family: segoe_ui.family,
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let mut col_top = column![];
    let mut col_rest = column![];
    
    let mut tela = row![];

    let mut times_ordenados: Vec<_> = lista_de_times.iter()
        .filter_map(|time| {
            time.pontos.iter()
                .find(|p| p.rodada == rodada)
                .map(|pontuacao| (time, pontuacao.pontos))
        })
        .collect();

    times_ordenados.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    if times_ordenados.is_empty() {
        tela = tela.push(text("Nenhum time encontrado na rodada atual."));
    } else {
        for (i, (time, pontos)) in times_ordenados.iter().enumerate() {
            if i < 5 {
                col_top = col_top.push(
                    row![
                        iced::widget::image::Image::new(
                            iced::widget::image::Handle::from_path(
                                format!("{}/assets/img/medal/lugar-{}.png", env!("CARGO_MANIFEST_DIR"), i + 1)
                            )
                        )
                        .height(80)
                        .width(100),
                        text(&time.nome_do_time)
                            .width(Length::Fill)
                            .size(60)
                            .font(segoe_ui_bold),
                        text(format!("{:.2}", pontos))
                            .width(Length::Fixed(200.0))
                            .size(60)
                            .style(move |_: &iced::Theme| text::Style {
                                color: Some(Color::from_rgb(0.2, 0.7, 0.2))
                            })
                            .align_x(iced::alignment::Horizontal::Right)
                            .font(segoe_ui_bold),
                    ].align_y(Alignment::Center)
                );
                if i < 4 {
                    col_top = col_top.push(Space::with_height(Length::Fill));
                }
            } else {
                col_rest = col_rest.push(
                    row![
                        if i > 4 {
                            Space::with_height(Length::Fill)
                        } else {
                            Space::with_height(0)
                        },
                        text(format!("{:>2}º", i + 1))
                            .width(Length::Fixed(80.0))
                            .size(20)
                            .align_x(Alignment::Center)
                            .font(segoe_ui),
                        text(&time.nome_do_time)
                            .width(Length::Fill)
                            .size(20)
                            .font(segoe_ui),
                        text(format!("{:.2}", pontos))
                            .width(Length::Fixed(80.0))
                            .size(20)
                            .style(move |_: &iced::Theme| text::Style {
                                color: Some(Color::from_rgb(0.2, 0.7, 0.2))
                            })
                            .align_x(iced::alignment::Horizontal::Right)
                            .font(segoe_ui),
                    ]
                );
            }
        }

        tela = tela.push(col_top).push(col_rest);
    }

    WhiteFrame::new(tela.spacing(50).padding(30).into()).into()
}
