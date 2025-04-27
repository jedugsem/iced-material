pub use iced::widget::overlay::menu::Style;
use iced::{
    widget::overlay::menu::{Catalog, StyleFn},
    Background, Border,
};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(primary)
    }

    fn style(&self, class: &StyleFn<'_, Self>) -> Style {
        class(self)
    }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        text_color: theme.colors().text.base,
        background: Background::Color(theme.colors().background.darker),
        border: Border {
            width: 1.0,
            radius: 4.0.into(),
            color: theme.colors().background.darker,
        },
        selected_text_color: theme.colors().text.low_alpha,
        selected_background: Background::Color(theme.colors().background.dark),
    }
}
