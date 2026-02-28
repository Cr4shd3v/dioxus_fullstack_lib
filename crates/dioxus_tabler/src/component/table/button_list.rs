use dioxus::prelude::*;

#[component]
pub fn TableButtonList(
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "btn-list flex-nowrap d-flex justify-content-end",
            {children}
        }
    }
}