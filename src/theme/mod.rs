pub mod button;
pub mod checkbox;
pub mod container;
pub mod data;
pub mod markdown;
pub mod menu;
pub mod pick_list;
pub mod scrollable;
pub mod svg;
pub mod text;
pub mod text_editor;
pub mod text_input;
pub mod toggler;
use data::ColorTheme;
use data::Colors;
use iced::color;
// TODO: If we use non-standard font sizes, we should consider
// Config.font.size since it's user configurable
pub const TEXT_SIZE: f32 = 13.0;
pub const ICON_SIZE: f32 = 12.0;

#[derive(Debug, Clone)]
pub enum Theme {
    Selected(ColorTheme),
    Preview {
        selected: ColorTheme,
        preview: ColorTheme,
    },
}

impl Theme {
    pub fn light() -> Self {
        let palette = data::Palette {
            accent: color!(0x1c71d8),
            success: color!(0x1b8553),
            error: color!(0xc01c28),
            background: color!(0xfafafa),
            alert: color!(0x9c6e03),
            text: color!(0x000000),
            //
            action: color!(0x62a0ea),
            info: color!(0x1b8553),
        };
        Theme::Selected(ColorTheme {
            name: String::from("light"),
            colors: Colors::new(&palette),
        })
    }
    pub fn dark() -> Self {
        let palette = data::Palette {
            accent: color!(0x78aeed),
            success: color!(0x8ff0a4),
            error: color!(0xff7b63),
            background: color!(0x242424),
            alert: color!(0xf8e45c),
            text: color!(0xffffff),
            //
            action: color!(0x99c1f1),
            info: color!(0x1b8553),
        };
        Theme::Selected(ColorTheme {
            name: String::from("light"),
            colors: Colors::new(&palette),
        })
    }
    pub fn preview(&self, theme: ColorTheme) -> Self {
        match self {
            Theme::Selected(selected) | Theme::Preview { selected, .. } => Self::Preview {
                selected: selected.clone(),
                preview: theme,
            },
        }
    }

    pub fn selected(&self) -> Self {
        match self {
            Theme::Selected(selected) | Theme::Preview { selected, .. } => {
                Self::Selected(selected.clone())
            }
        }
    }

    pub fn colors(&self) -> &Colors {
        match self {
            Theme::Selected(selected) => &selected.colors,
            Theme::Preview { preview, .. } => &preview.colors,
        }
    }
}

impl From<ColorTheme> for Theme {
    fn from(theme: ColorTheme) -> Self {
        Theme::Selected(theme)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::from(ColorTheme::default())
    }
}
