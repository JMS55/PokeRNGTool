use gtk::*;

pub fn view() -> Stack {
    let view_stack = Stack::new();
    view_stack
}

pub fn header_bar(
    main_view_stack: &Stack,
    header_bar_stack: &Stack,
    sub_view_stack: &Stack,
) -> HeaderBar {
    let back_button = Button::new_from_icon_name(Some("go-previous-symbolic"), IconSize::Button);
    back_button.connect_clicked({
        let main_view_stack = main_view_stack.clone();
        let header_bar_stack = header_bar_stack.clone();
        move |_| {
            main_view_stack.set_visible_child_name("main");
            header_bar_stack.set_visible_child_name("main");
        }
    });

    let stack_switcher = StackSwitcherBuilder::new().stack(sub_view_stack).build();

    let header_bar = HeaderBarBuilder::new().show_close_button(true).build();
    header_bar.set_custom_title(Some(&stack_switcher));
    header_bar.pack_start(&back_button);
    header_bar
}
