#[allow(unused_variables)]
use super::Theme;
use iced::widget::svg::{Catalog, Status, Style, StyleFn};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(_theme: &Theme, _status: Status) -> Style {
    Style::default()
}

pub fn error(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(theme.colors().error.base),
    }
}

pub fn success(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(theme.colors().success.base),
    }
}
