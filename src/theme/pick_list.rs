use iced::widget::pick_list::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(primary)
    }

    fn style(&self, class: &StyleFn<'_, Self>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, _status: Status) -> Style {
    Style {
        text_color: theme.colors().text.base,
        placeholder_color: theme.colors().background.dark,
        handle_color: theme.colors().background.darkest,
        background: Background::Color(theme.colors().background.darker),
        border: Border::default().rounded(5),
    }
}
