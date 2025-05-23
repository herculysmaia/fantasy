use iced::futures::future::ready;
use iced::{Color, Font, Task};
use iced::widget::{column, row, text, Image};
use iced::widget::image::Handle;

use crate::app::screen::finance::TimeData;
use crate::app::{obter_times, MessageDispatcher, Screen, ScreenTaskReturn, Time};

use super::super::common::WhiteFrame;
use super::{FinanceMessage, TimeDataMessage};

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
                let task = Task::perform(
                    ready(MessageDispatcher::Finance(
                        FinanceMessage::TimeData(TimeDataMessage::Save(time))
                    )),
                    |msg| msg
                );
                
                (Some(Box::new(TimeData::new())), task)
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
    let mut widget = column![];
    
    for time in lista {
        widget = widget.push(
            WhiteFrame::new(widget_time(time).into()).on_press(message_proc(InicioFinanceMessage::GoToEdit(time.clone())))
        );
    }

    widget.spacing(10).into()
}

fn widget_time(time: &Time) -> iced::Element<'static, MessageDispatcher> {
    column![
        row![
            Image::new(Handle::from_bytes(time.escudo_png.clone())),
            column![
                text(format!("{}", time.nome_do_time)),
                row![
                    text(format!("R$ {}", time.financeiro)),
                    text(format!("{}", time.nome_do_dono)),
                ]
            ]
        ]
    ].into()
}