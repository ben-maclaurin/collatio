pub fn input_component() -> impl Widget<u32> {
    let mut col = Flex::column();

    let input = TextBox::new().with_placeholder("test");
    col.add_child(input);
}