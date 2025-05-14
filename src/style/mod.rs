use iced::{theme::Palette, Color};

pub fn theme() -> iced::Theme {
    let palette = Palette {
        background: Color { r: 217.0/255.0, g: 99.0/255.0, b: 0.0/255.0, a: 255.0/255.0 },
        text: Color { r: 34.0/255.0, g: 34.0/255.0, b: 34.0/255.0, a: 255.0/255.0 },
        primary: Color { r: 255.0/255.0, g: 255.0/255.0, b: 255.0/255.0, a: 255.0/255.0 },
        success: Color { r: 38.0/255.0, g: 202.0/255.0, b: 94.0/255.0, a: 255.0/255.0 },
        danger: Color { r: 255.0/255.0, g: 6.0/255.0, b: 33.0/255.0, a: 255.0/255.0 }
    };

    iced::Theme::custom("Cartola".to_string(), palette)
}