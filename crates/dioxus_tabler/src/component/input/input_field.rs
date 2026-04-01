use dioxus::prelude::*;

#[component]
pub fn TextInputField(
    #[props(into)]
    name: String,
    #[props(into, optional)]
    value: Signal<String>,
    #[props(into, default="text")]
    input_type: String,
    #[props(into, optional)]
    label: String,
    #[props(into, optional)]
    label_description: Option<Element>,
    #[props(optional)]
    required: bool,
    #[props(into, optional)]
    placeholder: String,
    #[props(into, optional)]
    class: String,
    #[props(into, optional)]
    on_blur: EventHandler,
    #[props(into, optional)]
    on_input: EventHandler,
) -> Element {
    let input_id = format!("form_{name}");
    let mut label_class = "form-label".to_string();
    if required {
        label_class.push_str(" required");
    }

    let mut input_class = "form-control".to_string();
    if !class.is_empty() {
        input_class.push_str(&*class);
    }

    rsx! {
        label {
            for: input_id.clone(),
            class: label_class,
            {label}
            if let Some(description) = label_description {
                span {
                    class: "form-label-description",
                    {description}
                }
            }
        }
        input {
            id: input_id,
            name,
            type: input_type,
            class: input_class,
            required,
            placeholder,
            value,
            oninput: move |event| {
                value.set(event.value());
                on_input.call(());
            },
            onblur: move |_| {
                on_blur.call(());
            },
        }
    }
}