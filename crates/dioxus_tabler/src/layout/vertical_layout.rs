use dioxus::prelude::*;

#[component]
pub fn TablerVerticalLayout(
    #[props(into, optional)]
    title: String,
    #[props(into, optional)]
    pretitle: String,
    #[props(into, optional)]
    navbar: Option<Element>,
    #[props(into, optional)]
    actions: Option<Element>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "page",
            {navbar}

            div {
                class: "page-wrapper",
                div {
                    class: "container-fluid",
                    div {
                        class: "page-header d-print-none",
                        div {
                            class: "row g-2 align-items-center",
                            div {
                                class: "col",
                                div {
                                    class: "page-pretitle",
                                    {pretitle}
                                }
                                h2 {
                                    class: "page-title",
                                    {title}
                                }
                            }

                            div {
                                class: "col-auto ms-auto d-print-none",
                                div {
                                    class: "btn-list",
                                    {actions}
                                }
                            }
                        }
                    }
                }

                div {
                    class: "page-body",
                    div {
                        class: "container-fluid",
                        div {
                            class: "row row-cards",
                            section {
                                class: "content",
                                {children}
                            }
                        }
                    }
                }
            }
        }
    }
}