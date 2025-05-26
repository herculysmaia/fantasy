use iced::{widget::{button, column, combo_box, row, text, text_input}, Element, Length, Task};

use crate::app::{db::TipoMovimentacao, screen::{Finance, MessageDispatcher, Screen, ScreenTaskReturn}, Time};

use super::FinanceMessage;

#[derive(Debug, Clone)]
pub enum TimeDataMessage {
    OpenPopUp,
    ClosePopUp,
    TipoMovEscolhido(TipoMovimentacao),
    EntradaValor(String),
    EntradaData(String),
    Save,
    Back,
}

pub struct PopUpData {
    movimentacoes: combo_box::State<TipoMovimentacao>,
    movimentacao: Option<TipoMovimentacao>,
    valor: String,
    data: String,
}

pub struct TimeData {
    data: Time,
    show_popup: bool,
    popup_data: PopUpData,
}

fn message_proc(msg: TimeDataMessage) -> MessageDispatcher {
    MessageDispatcher::Finance(FinanceMessage::TimeData(msg))
}

impl TimeData {
    pub fn new(data: Time) -> Self {
        let opt = vec![
            TipoMovimentacao::Premiacao,
            TipoMovimentacao::Deposito,
            TipoMovimentacao::Retirada,
            TipoMovimentacao::Indicacao,
        ];

        let popup_date = PopUpData {
            movimentacoes: combo_box::State::new(opt),
            movimentacao: None, 
            valor: String::new(),
            data: String::new(),
        };

        Self { 
            data: data,
            show_popup: false,
            popup_data: popup_date,
        }
    }

    fn cabecalho(&self) -> Element<MessageDispatcher> {
        row![
            row![
                text(format!("#{}", self.data.id)),
                text(format!("{}", self.data.nome_do_time)),
                text(format!("{}", self.data.nome_do_dono)),
                ],
            text(format!("R$ {:.2}", self.data.financeiro.obter_saldo())),
        ].into()
    }

    fn content(&self) -> Element<MessageDispatcher> {
        column![
            row![
                row![],
                row![]
            ]
        ].into()
    }

    fn rodape(&self) -> Element<MessageDispatcher> {
        row![
            button(text("Adicionar")).on_press(
                message_proc(TimeDataMessage::OpenPopUp)
            ),
            button(text("Voltar")).on_press(
                message_proc(TimeDataMessage::Back)
            ),
        ].into()
    }

    fn pop_up(&self) -> Element<MessageDispatcher> {
        column![
            row![
                combo_box(&self.popup_data.movimentacoes, "Tipo de movimentação", self.popup_data.movimentacao.as_ref(), |tipo| message_proc(TimeDataMessage::TipoMovEscolhido(tipo))),
                text_input("Valor", &self.popup_data.valor)
                    .on_input(|s| message_proc(TimeDataMessage::EntradaValor(s))),
                text_input("Data (xx/xx)", &self.popup_data.data)
                    .on_input(|s| message_proc(TimeDataMessage::EntradaData(s))),
            ].width(Length::Fill).height(Length::Fill).spacing(10),
            row![
                button(text("Adicionar")).on_press(message_proc(TimeDataMessage::Save)),
                button(text("Close")).on_press(message_proc(TimeDataMessage::ClosePopUp)),
            ]
        ].into()
    }
}

impl Screen for TimeData {
    fn update(&mut self, message: MessageDispatcher) -> ScreenTaskReturn {
        use FinanceMessage::TimeData;
        use TimeDataMessage::*;

        let mut retorno: ScreenTaskReturn = (None, Task::none());

        match message {
            MessageDispatcher::Finance(TimeData(OpenPopUp)) => self.show_popup = true,
            MessageDispatcher::Finance(TimeData(ClosePopUp)) => self.show_popup = false,
            MessageDispatcher::Finance(TimeData(TipoMovEscolhido(tipo))) => self.popup_data.movimentacao = Some(tipo),
            MessageDispatcher::Finance(TimeData(EntradaValor(s))) => self.popup_data.valor = s,
            MessageDispatcher::Finance(TimeData(EntradaData(s))) => self.popup_data.data = s,
            
            MessageDispatcher::Finance(TimeData(Save)) => {
                let parts: Vec<&str> = self.popup_data.data.split("/").collect();

                let dia: u32 = parts[0].parse().unwrap();
                let mes: u32 = parts[1].parse().unwrap();

                let valor: f32 = self.popup_data.valor.parse().unwrap();
                let tipo = self.popup_data.movimentacao.clone();

                let tipo = if let Some(tipo) = tipo {
                    match tipo {
                        TipoMovimentacao::Premiacao => 0,
                        TipoMovimentacao::Deposito => 1,
                        TipoMovimentacao::Retirada => 2,
                        TipoMovimentacao::Indicacao => 3,
                        TipoMovimentacao::Desconhecida => 4,
                    }
                } else { 4 };

                self.data.salvar_movimentacao(
                    self.data.id,
                    dia,
                    mes,
                    valor,
                    tipo,
                );

                self.data.atulizar_financeiro();

                self.show_popup = false;
            }
            MessageDispatcher::Finance(TimeData(Back)) => retorno = (Some(Box::new(Finance::new())), Task::none()),
            _ => println!("Passou pelo update: TimeDataMessage com {:?}", message),
        }

        retorno
    }

    fn view(&self) -> Element<MessageDispatcher> {
        let content = if self.show_popup {
            column![
                self.pop_up(),
            ]
        } else {
            column![
                self.content(),
                self.rodape(),
            ]
        };
        
        column![
            self.cabecalho(),
            content
        ].width(Length::Fill).height(Length::Fill).spacing(10).into()
    }
}