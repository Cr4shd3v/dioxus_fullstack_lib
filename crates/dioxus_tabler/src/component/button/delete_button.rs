use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn DeleteButton(
    #[props(into)]
    id: Uuid,
    #[props(into)]
    object_name: String,
    #[props(into, optional)]
    on_delete: Callback<Uuid>,
) -> Element {
    let code = format!("return confirm('Do you really want to delete this {object_name}?')");

    rsx! {
        button {
            class: "btn btn-danger btn-icon",
            type: "button",
            onclick: move |_| {
                let code = code.clone();
                async move {
                    let result = document::eval(&*code).await.unwrap();
                    if result.as_bool().unwrap_or_default() {
                        on_delete(id);
                    }
                }
            },
            i {
                class: "fa-solid fa-trash"
            }
        }
    }
}