use dioxus::prelude::*;

#[component]
pub fn Card(
    #[props(into, optional)]
    title: Option<String>,
    #[props(into, optional)]
    class: Option<String>,
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let class = class.map(|v| format!("card {v}")).unwrap_or("card".to_string());

    rsx! {
        div {
            class,
            ..attributes,
            if title.is_some() {
                CardHeader {
                    title,
                }
            }
            div {
                class: "card-body",
                {children}
            }
        }
    }
}

#[component]
pub fn CardHeader(
    #[props(into, optional)]
    title: Option<String>,
) -> Element {
    rsx! {
        div {
            class: "card-header",
            h3 {
                class: "card-title",
                {title}
            }
        }
    }
}