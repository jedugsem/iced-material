use iced::widget::{svg, Svg};

pub mod header;
pub mod sidebar;
pub mod theme;

pub fn svg_icon<'a>(bytes: &'static [u8]) -> Svg<'a, theme::Theme> {
    svg(svg::Handle::from_memory(bytes))
}
use crate as iced_material;
#[macro_export]
macro_rules! icon {
    ($message_id:literal) => {{
        iced_material::svg_icon(include_bytes!(concat!(
            concat!("/home/me/workspace/lucide/icons/", $message_id),
            ".svg"
        )))
    }};
}
