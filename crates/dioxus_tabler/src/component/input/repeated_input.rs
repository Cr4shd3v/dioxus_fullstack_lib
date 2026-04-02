use dioxus::prelude::*;
use crate::TextInputField;

#[component]
pub fn RepeatedInputField(
    #[props(into, optional)]
    value: Signal<String>,
    #[props(into, default = "col-4")]
    container_class: String,
    #[props(into, optional)]
    required: bool,
    #[props(into, optional)]
    label: String,
    #[props(into)]
    name: String,
    #[props(into, default = "password")]
    input_type: String,
) -> Element {
    let repeat_label = if label.is_empty() { String::new() } else { format!("Repeat {label}") };
    let repeat_name = format!("{name}_repeat");
    let value1 = use_signal(|| value.read().cloned());
    let value2 = use_signal(|| value.read().cloned());
    let mut error = use_signal(|| String::new());

    use_effect(move || {
        if *value1.read() != *value2.read() {
            error.set("Inputs do not match!".to_string());
        } else {
            error.set(String::new());
            value.set(value1.cloned());
        }
    });

    rsx! {
        div {
            class: container_class.clone(),
            TextInputField {
                name,
                input_type: input_type.clone(),
                label,
                required,
                value: value1,
                invalid_message: error,
            },
        }
        div {
            class: container_class,
            TextInputField {
                name: repeat_name,
                input_type,
                label: repeat_label,
                required,
                value: value2,
                invalid_message: error,
            },
        }
    }
}
