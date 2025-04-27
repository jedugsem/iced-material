use iced::{
    widget::text_input::{Catalog, Status, Style, StyleFn},
    Border,
};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(none)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn none(_theme: &Theme, _state: Status) -> Style {
    Style {
        value: _theme.colors().text.base,
        selection: _theme.colors().text.light,
        icon: _theme.colors().text.base,
        placeholder: _theme.colors().text.lighter,
        border: Border::default().rounded(10.),
        background: iced::Background::from(_theme.colors().background.base),
    }
}
