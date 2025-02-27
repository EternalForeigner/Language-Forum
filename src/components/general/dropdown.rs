use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    button: Element,
    menu: Element
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {

    rsx!{
        div { class: "group",
            {props.button}
            div { class: "absolute opacity-0 invisible right-0 overflow-auto group-focus-within:opacity-100 group-focus-within:visible transform transition-all duration-300 ease-in-out -translate-y-2 group-focus-within:translate-y-0 mt-2",
                {props.menu}
            }
        }
    }
}
