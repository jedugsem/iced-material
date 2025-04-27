use iced::widget::toggler::{Catalog, Status, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: iced::widget::toggler::Status) -> Style {
        class(self, status)
    }
}

pub fn primary(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: _theme.colors().background.darkest,
        background_border_width: 10.0,
        background_border_color: _theme.colors().background.darkest,
        foreground: _theme.colors().text.lightest,
        foreground_border_width: 10.0,
        foreground_border_color: _theme.colors().text.high_alpha,
    }
}
