use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LoaderProps {

    #[props(default = LoaderType::Spinner)]
    loader_type: LoaderType,


    #[props(default = LoaderSize::Medium)]
    size: LoaderSize,


    #[props(default)]
    text: Option<String>,


    #[props(default)]
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoaderType {
    Spinner,
    Dots,
    Bars,
    Pulse,
    Slide,
    Ring,
    Wave,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoaderSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn Loader(props: LoaderProps) -> Element {
    let mut classes = vec!["loader"];


    match props.loader_type {
        LoaderType::Spinner => classes.push("loader-spinner"),
        LoaderType::Dots => classes.push("loader-dots"),
        LoaderType::Bars => classes.push("loader-bars"),
        LoaderType::Pulse => classes.push("loader-pulse"),
        LoaderType::Slide => classes.push("loader-slide"),
        LoaderType::Ring => classes.push("loader-ring"),
        LoaderType::Wave => classes.push("loader-wave"),
        LoaderType::Custom => classes.push("loader-custom"),
    }


    match props.size {
        LoaderSize::Small => classes.push("loader-small"),
        LoaderSize::Medium => classes.push("loader-medium"),
        LoaderSize::Large => classes.push("loader-large"),
    }

    if let Some(class) = &props.class {
        classes.push(class);
    }

    let final_class = classes.join(" ");


    let loading_text = props
        .text
        .clone()
        .unwrap_or_else(|| "Loading...".to_string());

    rsx! {
        div {
            class: final_class,
            ..props.attributes,


            match props.loader_type {
                LoaderType::Spinner => rsx! {
                    div { class: "loader-spinner-inner" }
                    if props.text.is_some() {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Dots => rsx! {
                    div { class: "loader-dots-container",
                        div { class: "loader-dot" }
                        div { class: "loader-dot" }
                        div { class: "loader-dot" }
                    }
                    if props.text.is_some() {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Bars => rsx! {
                    div { class: "loader-bars-container",
                        div { class: "loader-bar" }
                        div { class: "loader-bar" }
                        div { class: "loader-bar" }
                        div { class: "loader-bar" }
                    }
                    if props.text.is_some() {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Pulse => rsx! {
                    div { class: "loader-pulse-container",
                        div { class: "loader-pulse-inner" }
                    }
                    if props.text.is_some() {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Slide => rsx! {
                    div { class: "loader-slide-container",
                        div { class: "loader-slide-inner" }
                    }
                    if props.text.is_some() {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Ring => rsx! {
                    div { class: "loader-ring-container",
                        div { class: "loader-ring-inner" }
                    }
                    if props.text.is_some() {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Wave => rsx! {
                    div { class: "loader-wave-container",
                        div { class: "loader-wave-inner" }
                    }
                    if props.text.is_some() {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Custom => rsx! {
                    div { class: "loader-custom-container",
                        div { class: "loader-custom-inner" }
                    }
                    if props.text.is_some() {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PageLoaderProps {

    #[props(default = true)]
    visible: bool,


    #[props(default = LoaderType::Spinner)]
    loader_type: LoaderType,


    #[props(default)]
    text: Option<String>,


    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn PageLoader(props: PageLoaderProps) -> Element {
    if !props.visible {
        return rsx! { div { style: "display: none;" } };
    }

    let mut classes = vec!["page-loader"];
    if let Some(class) = &props.class {
        classes.push(class);
    }
    let final_class = classes.join(" ");

    rsx! {
        div {
            class: final_class,
            Loader {
                loader_type: props.loader_type,
                size: LoaderSize::Large,
                text: props.text,
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InlineLoaderProps {

    #[props(default = LoaderType::Pulse)]
    loader_type: LoaderType,


    #[props(default = LoaderSize::Small)]
    size: LoaderSize,


    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn InlineLoader(props: InlineLoaderProps) -> Element {
    rsx! {
        Loader {
            loader_type: props.loader_type,
            size: props.size,
            class: props.class,
        }
    }
}
