mod component;

use component::input::input_component;
use druid::piet::Color;
use druid::widget::{Container, Flex};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};

const WINDOW_TITLE: LocalizedString<ApplicationState> = LocalizedString::new("Collatio");

#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    username: String,
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    let application_state = ApplicationState {
        username: "".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(application_state)
        .expect("failed to launch");
}

// This is where the application starts
fn build_root_widget() -> impl Widget<ApplicationState> {
    let column = Flex::column().with_child(input_component("test".into()));
    let mut container = Container::new(column);
    container.set_background(Color::WHITE);
    container
}
