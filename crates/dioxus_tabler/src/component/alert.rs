use dioxus::prelude::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum AlertType {
    #[default]
    Danger,
    Warning,
    Success,
    Info,
    Custom {
        type_str: String,
        icon_class: String,
    },
}

impl AlertType {
    fn get_type_str(&self) -> String {
        match self {
            AlertType::Danger => "danger",
            AlertType::Warning => "warning",
            AlertType::Success => "success",
            AlertType::Info => "info",
            AlertType::Custom { type_str, .. } => type_str,
        }.to_string()
    }

    fn get_icon_class(&self) -> String {
        match self {
            AlertType::Danger => "fa-circle-exclamation",
            AlertType::Warning => "fa-triangle-exclamation",
            AlertType::Success => "fa-check",
            AlertType::Info => "fa-circle-info",
            AlertType::Custom { icon_class, .. } => icon_class,
        }.to_string()
    }
}

#[component]
pub fn Alert(
    #[props(into, optional)]
    alert_type: AlertType,
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: format!("alert alert-{}", alert_type.get_type_str()),
            role: "alert",
            ..attributes,
            div {
                class: "alert-icon",
                i {
                    class: format!("fa-solid {}", alert_type.get_icon_class()),
                }
            }
            {children}
        }
    }
}