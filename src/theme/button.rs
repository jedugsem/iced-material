use iced::widget::button::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border, Color};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(text_button)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}
pub fn succes(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(theme.colors().accent.high_alpha)),
            text_color: theme.colors().accent.base,
            border: Border {
                radius: 3.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Background::Color(theme.colors().accent.med_alpha)),
            text_color: theme.colors().accent.base,
            border: Border {
                radius: 3.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Disabled => {
            let active = secondary(theme, Status::Active);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                border: Border {
                    color: Color {
                        a: 0.2,
                        ..active.text_color
                    },
                    ..Default::default()
                },
                ..active
            }
        }
    }
}

pub fn text_button(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(theme.colors().text.high_alpha)),
            text_color: theme.colors().text.base,
            border: Border {
                radius: 10.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Background::Color(theme.colors().text.med_alpha)),
            text_color: theme.colors().text.base,
            border: Border {
                radius: 10.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Disabled => {
            let active = text_button(theme, Status::Active);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                border: Border {
                    radius: 10.0.into(),
                    color: Color {
                        a: 0.2,
                        ..active.text_color
                    },
                    ..Default::default()
                },
                ..active
            }
        }
    }
}

pub fn icon_button(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(theme.colors().text.high_alpha)),
            text_color: theme.colors().text.base,
            border: Border {
                radius: 10.0.into(),
                width: 0.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Background::Color(theme.colors().text.med_alpha)),
            text_color: theme.colors().text.base,
            border: Border {
                radius: 10.0.into(),
                width: 0.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Disabled => {
            let active = text_button(theme, Status::Active);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                border: Border {
                    color: Color {
                        a: 0.2,
                        ..active.text_color
                    },
                    width: 0.0.into(),
                    ..Default::default()
                },
                ..active
            }
        }
    }
}

pub fn unselected(theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Background::Color(theme.colors().background.dark)),
        text_color: theme.colors().text.base,
        border: Border {
            radius: 10.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn selected(theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Background::Color(theme.colors().background.darkest)),
        text_color: theme.colors().text.base,
        border: Border {
            radius: 10.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
pub fn secondary(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(theme.colors().background.dark)),
            text_color: theme.colors().accent.base,
            border: Border {
                radius: 10.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Background::Color(theme.colors().background.lightest)),
            text_color: theme.colors().accent.base,
            border: Border {
                radius: 10.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Disabled => {
            let active = secondary(theme, Status::Active);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                border: Border {
                    color: Color {
                        a: 0.2,
                        ..active.text_color
                    },
                    ..Default::default()
                },
                ..active
            }
        }
    }
}

pub fn tertiary(theme: &Theme, status: Status, selected: bool) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(if selected {
                theme.colors().action.med_alpha
            } else {
                theme.colors().background.dark
            })),
            border: Border {
                color: if selected {
                    theme.colors().action.low_alpha
                } else if theme.colors().is_dark_theme() {
                    theme.colors().background.lightest
                } else {
                    theme.colors().background.darkest
                },
                width: 1.0,
                radius: 3.0.into(),
            },
            ..Default::default()
        },
        Status::Hovered => {
            let active = tertiary(theme, Status::Active, selected);

            Style {
                background: Some(Background::Color(if selected {
                    theme.colors().action.high_alpha
                } else if theme.colors().is_dark_theme() {
                    theme.colors().background.light
                } else {
                    theme.colors().background.darker
                })),
                ..active
            }
        }
        Status::Disabled => {
            let active = tertiary(theme, Status::Active, selected);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                border: Border {
                    color: Color {
                        a: 0.2,
                        ..active.text_color
                    },
                    ..Default::default()
                },
                ..active
            }
        }
    }
}

pub fn context(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Hovered => {
            let active = context(theme, Status::Active);

            Style {
                background: Some(Background::Color(theme.colors().background.darker)),
                ..active
            }
        }
        Status::Disabled => {
            let active = context(theme, Status::Active);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                border: Border {
                    color: Color {
                        a: 0.2,
                        ..active.text_color
                    },
                    ..Default::default()
                },
                ..active
            }
        }
    }
}

pub fn bare(_theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed | Status::Hovered => Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        },
        Status::Disabled => {
            let active = bare(_theme, Status::Active);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                border: Border {
                    color: Color {
                        a: 0.2,
                        ..active.text_color
                    },
                    ..Default::default()
                },
                ..active
            }
        }
    }
}

pub fn side_menu(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: None,
            border: Border {
                radius: 3.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Hovered => {
            let active = side_menu(theme, Status::Active);

            Style {
                background: Some(Background::Color(theme.colors().background.dark)),
                ..active
            }
        }
        Status::Disabled => {
            let active = side_menu(theme, Status::Active);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                border: Border {
                    color: Color {
                        a: 0.2,
                        ..active.text_color
                    },
                    ..Default::default()
                },
                ..active
            }
        }
    }
}

pub fn side_menu_selected(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(theme.colors().background.darker)),
            border: Border {
                radius: 3.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Hovered => {
            let active = side_menu_selected(theme, Status::Active);

            Style {
                background: Some(Background::Color(theme.colors().background.darkest)),
                ..active
            }
        }
        Status::Disabled => {
            let active = side_menu_selected(theme, Status::Active);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                border: Border {
                    color: Color {
                        a: 0.2,
                        ..active.text_color
                    },
                    ..Default::default()
                },
                ..active
            }
        }
    }
}
