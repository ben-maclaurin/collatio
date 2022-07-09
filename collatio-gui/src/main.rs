mod component;

use druid::widget::{Label, Button, Flex};
use druid::{AppLauncher, PlatformError, WindowDesc, Widget};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

// This is where the application starts
fn ui_builder() -> impl Widget<u32> {
    let button = Button::new("hello");

    Flex::column().with_child(button)
}
