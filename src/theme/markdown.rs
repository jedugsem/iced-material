use crate::theme::container::grey;
use iced::{
    widget::{
        container,
        markdown::{self, Catalog, Style},
    },
    Border, Padding,
};

use super::Theme;

impl Catalog for Theme {
    fn code_block<'a>() -> <Self as container::Catalog>::Class<'a> {
        Box::new(grey)
    }
}
pub fn light_grey(theme: &Theme) -> Style {
    Style {
        link_color: theme.colors().success.dark,
        inline_code_padding: Padding::from(5),
        inline_code_color: theme.colors().text.base,
        inline_code_highlight: iced::advanced::text::Highlight {
            background: iced::Background::Color(theme.colors().background.base),
            border: Border::default().rounded(5),
        },
    }
}
impl From<&Theme> for markdown::Settings {
    fn from(value: &Theme) -> Self {
        markdown::Settings::with_style(light_grey(value))
    }
}
