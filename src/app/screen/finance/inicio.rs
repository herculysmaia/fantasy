use iced::{Color, Font, Length, Task};
use iced::widget::{column, row, text, Image, Space};
use iced::widget::image::Handle;

use crate::app::screen::finance::TimeData;
use crate::app::{obter_times, MessageDispatcher, Screen, ScreenTaskReturn, Time};

use super::super::common::WhiteFrame;
use super::FinanceMessage;

#[derive(Debug, Clone)]
pub enum InicioFinanceMessage {
    GoToEdit(Time),
}

struct InicioFinanceData {
    times: Vec<Time>,
}

impl InicioFinanceData {
    fn new() -> Self {
        Self {times: obter_times() }
    }
}

pub struct InicioFinance{
    data: InicioFinanceData,
}

fn message_proc(msg: InicioFinanceMessage) -> MessageDispatcher {
    MessageDispatcher::Finance(FinanceMessage::Inicio(msg))
}

impl InicioFinance {
    pub fn new() -> Self {
        Self {
            data: InicioFinanceData::new(),
        }
    }
}

impl Screen for InicioFinance {
    fn update(&mut self, message: MessageDispatcher ) -> ScreenTaskReturn {
        use FinanceMessage::Inicio;
        use InicioFinanceMessage::*;

        match message {
            MessageDispatcher::Finance(Inicio(GoToEdit(time))) => {  
                (Some(Box::new(TimeData::new(time))), Task::none())
            }
            _ => (None, Task::none()),
        }
    }

    fn view(&self) -> iced::Element<MessageDispatcher> {
        column![
            titulo(),
            listar_times(&self.data.times),
        ].padding(10).spacing(10).into()
    }
}

fn titulo() -> iced::Element<'static, MessageDispatcher> {
    text("PARTICIPANTES").font(Font {
                weight: iced::font::Weight::Bold,
                ..Font::DEFAULT
            }).style(|_: &iced::Theme| text::Style {
                color: Some(Color::from_rgb(1., 1., 1.))
            }).into()
}

fn listar_times(lista: &Vec<Time>) -> iced::Element<'static, MessageDispatcher> {
    let mut linhas = column![].spacing(10);
    
    for linha in lista.chunks(6) {
        let mut linha_ui = row![].spacing(10);

        for time in linha {
            let item = 
                WhiteFrame::new(widget_time(time).into())
                .on_press(message_proc(InicioFinanceMessage::GoToEdit(time.clone())));

            linha_ui = linha_ui.push(item);
        }

        linhas = linhas.push(linha_ui);
    }

    linhas.into()
}

fn widget_time(time: &Time) -> iced::Element<'static, MessageDispatcher> {
    let segoe_ui = Font::with_name("Segoe UI");
    let segoe_ui_bold = Font {
        family: segoe_ui.family,
        weight: iced::font::Weight::Bold,
        ..Default::default()
    };

    let mut lista = column![];

    for movimentacao in time.financeiro.obter_movimentacoes() {
        lista = lista.push(
            row![
                text("Depósitos")
                    .font(segoe_ui)
                    .size(12),
                Space::with_width(Length::Fill),
                text(format!("R$ {:.2}", movimentacao.valor as f32 / 100.0))
                    .font(segoe_ui)
                    .size(12),
                ]
        )
    }

    column![
        row![
            Image::new(Handle::from_bytes(time.escudo_png.clone())).height(50),
            column![
                text(format!("{}", time.nome_do_time)).font(segoe_ui_bold),
                row![
                    text(format!("R$ {:.2}", time.financeiro.obter_saldo() as f32)).font(segoe_ui_bold).size(20),
                    Space::with_width(Length::Fill),
                    text(format!("{}", time.nome_do_dono)).font(segoe_ui).size(10),
                ].width(Length::Fill)
            ]
        ].spacing(5).padding(5),
        lista.padding(5)
    ].into()
}