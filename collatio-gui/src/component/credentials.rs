use crate::ApplicationState;
use druid::widget::TextBox;
use druid::{Widget, WidgetExt};

pub fn email_input() -> impl Widget<ApplicationState> {
    let input = TextBox::new()
        .with_placeholder("Email address ...")
        .lens(ApplicationState::email);

    input
}

pub fn password_input() -> impl Widget<ApplicationState> {
    let input = TextBox::new()
    .with_placeholder("Password ...")
    .lens(ApplicationState::password);

    input
}
