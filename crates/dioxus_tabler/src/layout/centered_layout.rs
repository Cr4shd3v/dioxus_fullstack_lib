use dioxus::prelude::*;

#[component]
pub fn CenteredLayout(
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "page page-center",
            div {
                class: "container py-4",
                {children}
            }
        }
    }
}