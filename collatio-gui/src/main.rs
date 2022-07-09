mod component;

use component::credentials::{email_input, password_input};
use druid::piet::Color;
use druid::widget::{Container, Flex};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};

const WINDOW_TITLE: LocalizedString<ApplicationState> = LocalizedString::new("Collatio");

#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    email: String,
    password: String,
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    let application_state = ApplicationState {
        email: "".into(),
        password: "".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(application_state)
        .expect("failed to launch");
}

// This is where the application starts
fn build_root_widget() -> impl Widget<ApplicationState> {
    let column = Flex::column().with_child(email_input()).with_child(password_input());
    let mut container = Container::new(column);
    container.set_background(Color::WHITE);
    container
}
