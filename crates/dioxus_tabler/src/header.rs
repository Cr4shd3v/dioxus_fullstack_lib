use dioxus::prelude::*;

#[component]
pub fn TablerHeader(
    children: Element,
) -> Element {
    rsx! {
        header {
            "data-bs-theme": "dark",
            class: "navbar d-none d-lg-block navbar-expland-md sticky-top d-print-none",
            div {
                class: "container-fluid justify-content-lg-end",
                div {
                    class: "navbar-nav flex-row order-md-last",
                    {children}
                }
            }
        }
    }
}