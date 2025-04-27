use iced::{
    alignment::Alignment,
    widget::{button, column, container, scrollable, text},
    Element, Length,
};
pub fn sidebar<'a, Message, Renderer>(
    names: &[&str],
    f: impl Fn(u8) -> Message + 'a,
) -> Element<'a, Message, crate::theme::Theme, Renderer>
where
    Message: 'a + Clone,
    // Theme: 'a
    //     + text::Catalog
    //     + button::Catalog
    //     + svg::Catalog
    //     + container::Catalog
    //     + scrollable::Catalog,
    Renderer: 'a + iced::advanced::text::Renderer,
{
    let options: Vec<Element<'_, _, _, _>> = names
        .iter()
        .enumerate()
        .map(|(i, title)| {
            // hey
            button(container(text!("{}", title)).center_y(Length::Fill))
                .width(Length::Fill)
                .height(50)
                .on_press(f(i as u8))
                // .style(move |theme, status| {
                //     if i as u8 == self.selected {
                //         text_button(theme, button::Status::Pressed)
                //     } else {
                //         text_button(theme, status)
                //     }
                // })
                .into()
        })
        .collect();

    container(scrollable(
        column(options)
            .padding(10)
            .align_x(Alignment::Center)
            .spacing(10),
    ))
    .height(Length::Fill)
    .center_x(Length::Fill)
    .style(crate::theme::container::light_grey)
    .into()
    //
}
