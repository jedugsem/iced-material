use iced::widget::container::{transparent, Catalog, Style, StyleFn};
use iced::{Background, Border, Color};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn secondary_rounded(theme: &Theme) -> Style {
    let background = theme.colors().accent.darkest;

    Style {
        background: Some(Background::Color(background)),
        text_color: Some(theme.colors().text.base),
        border: Border::default().rounded(10),
        ..Default::default()
    }
}
pub fn grey_rounded(theme: &Theme) -> Style {
    let background = theme.colors().background.darker;

    Style {
        background: Some(Background::Color(background)),
        text_color: Some(theme.colors().text.base),
        border: Border::default().rounded(10),
        ..Default::default()
    }
}

pub fn grey(theme: &Theme) -> Style {
    let background = theme.colors().background.darker;

    Style {
        background: Some(Background::Color(background)),
        text_color: Some(theme.colors().text.base),
        ..Default::default()
    }
}

pub fn light_grey(theme: &Theme) -> Style {
    let background = theme.colors().background.dark;

    Style {
        background: Some(Background::Color(background)),
        text_color: Some(theme.colors().text.base),
        ..Default::default()
    }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(theme.colors().background.base)),
        text_color: Some(theme.colors().text.base),
        ..Default::default()
    }
}

pub fn pane_body(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(theme.colors().background.dark)),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

pub fn pane_body_selected(theme: &Theme) -> Style {
    let pane_body = pane_body(theme);

    Style {
        border: Border {
            color: theme.colors().action.base,
            ..pane_body.border
        },
        ..pane_body
    }
}

pub fn command(_theme: &Theme) -> Style {
    Style {
        background: None,
        ..Default::default()
    }
}

pub fn command_selected(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(theme.colors().background.darker)),
        border: Border {
            radius: 3.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn context(theme: &Theme) -> Style {
    Style {
        //TODO: Blur background when possible?
        background: Some(Background::Color(theme.colors().background.base)),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: theme.colors().background.darker,
        },
        ..Default::default()
    }
}

pub fn highlight(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(theme.colors().info.high_alpha)),
        border: Border {
            radius: 0.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn semi_transparent(theme: &Theme) -> Style {
    Style {
        background: Some(
            Color {
                a: 0.80,
                ..theme.colors().background.base
            }
            .into(),
        ),
        ..Default::default()
    }
}

pub fn default_banner(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(theme.colors().background.dark)),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: theme.colors().background.lighter,
        },
        ..Default::default()
    }
}

pub fn error_modal(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(theme.colors().background.dark)),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: theme.colors().error.base,
        },
        ..Default::default()
    }
}
