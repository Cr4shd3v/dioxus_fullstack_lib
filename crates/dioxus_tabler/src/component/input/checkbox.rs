use dioxus::prelude::*;

#[component]
pub fn Checkbox(
    #[props(into)]
    name: String,
    #[props(into, optional)]
    value: Signal<bool>,
    #[props(into, optional)]
    label: String,
    #[props(into, optional)]
    required: bool,
) -> Element {
    let input_id = format!("form_{name}");
    let mut label_class = "form-check-label".to_string();
    if required {
        label_class.push_str(" required");
    }

    rsx! {
        div {
            class: "form-check",
            input {
                id: input_id.clone(),
                name,
                type: "checkbox",
                class: "form-check-input",
                required,
                checked: value,
                oninput: move |event| value.set(event.checked()),
            }
            label {
                for: input_id,
                class: label_class,
                {label}
            }
        }
    }
}