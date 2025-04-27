use iced::widget::text::{Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(none)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn none(_theme: &Theme) -> Style {
    Style { color: None }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().text.base),
    }
}

pub fn accent(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().accent.base),
    }
}

pub fn alert(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().alert.base),
    }
}

pub fn info(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().info.base),
    }
}

pub fn error(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().error.base),
    }
}

pub fn success(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().success.base),
    }
}

pub fn transparent(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().text.low_alpha),
    }
}

pub fn transparent_accent(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().accent.low_alpha),
    }
}
