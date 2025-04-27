use iced::{
    widget::checkbox::{Catalog, Status, Style, StyleFn},
    Border,
};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: iced::widget::checkbox::Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    match status {
        _ => Style {
            background: iced::Background::Color(theme.colors().background.base),
            icon_color: theme.colors().accent.base,
            border: Border {
                color: theme.colors().accent.base,
                width: 1.0,
                radius: 2.into(),
            },
            text_color: Some(theme.colors().text.base),
        },
    }
}
