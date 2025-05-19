use iced::{theme::Palette, Color};

fn cor(r: f32, g: f32, b: f32) -> Color {
    Color {
        r: r / 255.0,
        g: g / 255.0,
        b: b / 255.0,
        a: 1.0,
    }
}

pub fn theme() -> iced::Theme {
    let palette = Palette {
        background: cor(217.0,  99.0, 0.0),
        text: cor(34.0,  34.0, 34.0),
        primary: cor(255.0,  255.0, 255.0),
        success: cor(38.0,  202.0, 94.0),
        danger: cor(255.0,  6.0, 33.0)
    };

    iced::Theme::custom("Cartola".to_string(), palette)
}