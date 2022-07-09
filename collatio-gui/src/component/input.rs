use crate::ApplicationState;
use druid::widget::TextBox;
use druid::{Widget, WidgetExt};

pub fn input_component(placeholder: String) -> impl Widget<ApplicationState> {
    let input = TextBox::new()
        .with_placeholder(placeholder)
        .lens(ApplicationState::username);

    input
}
