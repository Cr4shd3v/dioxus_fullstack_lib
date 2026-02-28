use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct NavbarItem<T: Routable + PartialEq + Clone> {
    pub title: String,
    pub path: String,
    pub route: Option<T>,
    pub icon_classes: String,
    pub children: Vec<NavbarItem<T>>,
}

impl<T: Routable + PartialEq + Clone> NavbarItem<T> {
    pub fn new(title: impl Into<String>, route: T, icon_classes: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            path: route.to_string(),
            route: Some(route),
            icon_classes: icon_classes.into(),
            children: vec![],
        }
    }

    pub fn new_parent(title: impl Into<String>, path: impl Into<String>, icon_classes: impl Into<String>, builder: fn(&mut Vec<NavbarItem<T>>)) -> Self {
        let mut children = vec![];
        builder(&mut children);

        Self {
            title: title.into(),
            path: path.into(),
            route: None,
            icon_classes: icon_classes.into(),
            children,
        }
    }
}

#[component]
pub fn TablerNavbar<T: Routable + PartialEq>(
    #[props(into, optional)]
    logo_path: Option<String>,
    home_route: T,
    children: Element,
) -> Element {
    rsx! {
        aside {
            class: "navbar navbar-vertical navbar-expand-lg",
            "data-bs-theme": "dark",

            div {
                class: "container-fluid",

                if logo_path.is_some() {
                    div {
                        class: "navbar-brand",
                        Link {
                            to: home_route,
                            img {
                                class: "navbar-brand-image",
                                src: logo_path,
                            }
                        }
                    }
                }

                div {
                    class: "collapse navbar-collapse",
                    id: "navbar-menu",
                    ul {
                        class: "navbar-nav pt-lg-3",
                        {children}
                    }
                }
            }
        }
    }
}

#[component]
pub fn TablerNavItem<T: Routable + PartialEq>(
    item: NavbarItem<T>,
) -> Element {
    let route = use_route::<T>().to_string();
    let cloned_path = item.path.clone();
    let active = use_signal(move || route.starts_with(&cloned_path));
    let has_children = !item.children.is_empty();

    if has_children {
        rsx! {
            li {
                class: "nav-item dropdown",
                class: if active() { "active" },
                a {
                    class: "nav-link dropdown-toggle",
                    class: if active() { "show" },
                    href: "#",
                    "data-bs-toggle": "dropdown",
                    "data-bs-auto-close": "false",
                    role: "button",
                    aria_expanded: active(),
                    if !item.icon_classes.is_empty() {
                        span {
                            class: "nav-link-icon d-md-none d-lg-inline-block",
                            i {
                                class: item.icon_classes,
                            }
                        }
                    }
                    span {
                        class: "nav-link-title",
                        {item.title}
                    }
                }

                div {
                    class: "dropdown-menu",
                    class: if active() { "show" },
                    "data-bs-popper": "static",
                    div {
                        class: "dropdown-menu-columns",
                        div {
                            class: "dropdown-menu-column",
                            for item in item.children.into_iter() {
                                DropdownItem<T> {
                                    item,
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            li {
                class: "nav-item",
                class: if active() { "active" },
                Link {
                    class: "nav-link",
                    to: item.route.unwrap(),
                    if !item.icon_classes.is_empty() {
                        span {
                            class: "nav-link-icon d-md-none d-lg-inline-block",
                            i {
                                class: item.icon_classes,
                            }
                        }
                    }
                    span {
                        class: "nav-link-title",
                        {item.title}
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownItem<T: Routable + PartialEq>(
    item: NavbarItem<T>,
) -> Element {
    let route = use_route::<T>().to_string();
    let cloned_path = item.path.clone();
    let active = use_signal(move || route.starts_with(&cloned_path));

    rsx! {
        Link {
            class: if active() { "active dropdown-item" } else { "dropdown-item" },
            to: item.route.unwrap(),
            if !item.icon_classes.is_empty() {
                span {
                    class: "nav-link-icon d-md-none d-lg-inline-block",
                    i {
                        class: item.icon_classes,
                    }
                }
            }
            span {
                class: "nav-link-title",
                {item.title}
            }
        }
    }
}