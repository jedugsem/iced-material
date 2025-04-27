use crate::iced_material;
use crate::icon;
use iced::{
    alignment::Alignment,
    widget::{button, container, row, text, Space},
    Element, Length, Padding,
};
//pub const fun stri
pub fn header<'a, Message, Renderer>(
    side_button: Message,
    back_message: Message,
    settings_button: Message,
    exit_button: Message,
    //page: &Page,
    title: &str,
) -> Element<'a, Message, crate::theme::Theme, Renderer>
where
    Message: 'a + Clone,
    //Theme: 'a + text::Catalog + button::Catalog + svg::Catalog + container::Catalog,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
{
    container(
        row![
            row!(
                button(icon!("panel-left"))
                    .padding(4)
                    .width(30)
                    .height(30)
                    .on_press(side_button),
                button(icon!("chevron-left"))
                    .padding(4)
                    .width(30)
                    .height(30)
                    .on_press(back_message),
            )
            .spacing(10),
            Space::with_width(Length::Fill),
            text!("{}", title),
            Space::with_width(Length::Fill),
            row!(
                button(icon!("settings"))
                    .padding(4)
                    .on_press(settings_button)
                    .width(30)
                    .height(30),
                button(icon!("x"))
                    .padding(4)
                    .on_press(exit_button)
                    .width(30)
                    .height(30)
            )
            .spacing(10),
        ]
        .padding({
            if cfg!(target_os = "android") {
                Padding {
                    top: 40.,
                    left: 10.,
                    right: 10.,
                    bottom: 10.,
                }
            } else {
                Padding::new(10.)
            }
        })
        .align_y(Alignment::Center)
        .spacing(10),
    )
    .style(crate::theme::container::grey)
    .into()
}
