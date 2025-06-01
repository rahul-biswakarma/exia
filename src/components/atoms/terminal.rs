use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TerminalPanelProps {

    title: String,


    #[props(default)]
    color_theme: Option<String>,


    #[props(default)]
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TerminalPanel(props: TerminalPanelProps) -> Element {
    let mut classes = vec!["terminal-panel"];

    let theme_class = if let Some(theme) = &props.color_theme {
        format!("terminal-{}", theme)
    } else {
        String::new()
    };

    if !theme_class.is_empty() {
        classes.push(&theme_class);
    }

    if let Some(class) = &props.class {
        classes.push(class);
    }

    let final_class = classes.join(" ");

    rsx! {
        div {
            class: final_class,
            ..props.attributes,

            div {
                class: "terminal-panel-header",
                "[ {props.title.to_uppercase()} ]"
            }

            div {
                class: "terminal-panel-content",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct StatusIndicatorProps {

    status: String,


    status_type: String,


    #[props(default)]
    label: Option<String>,
}

#[component]
pub fn StatusIndicator(props: StatusIndicatorProps) -> Element {
    let status_class = format!("status-indicator status-{}", props.status_type);

    rsx! {
        div {
            class: status_class,
            if let Some(label) = &props.label {
                span { class: "status-label", "{label}: " }
            }
            span { class: "status-text", "{props.status}" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DataRowProps {

    label: String,


    value: String,


    #[props(default)]
    value_type: Option<String>,
}

#[component]
pub fn DataRow(props: DataRowProps) -> Element {
    let mut value_classes = vec!["data-value"];

    if let Some(value_type) = &props.value_type {
        value_classes.push(value_type);
    }

    let value_class = value_classes.join(" ");

    rsx! {
        div {
            class: "data-row",
            span { class: "data-label", "{props.label}" }
            span { class: value_class, "{props.value}" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SystemStatsProps {

    stats: Vec<(String, String, Option<String>)>,


    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn SystemStats(props: SystemStatsProps) -> Element {
    let mut classes = vec!["system-stats"];

    if let Some(class) = &props.class {
        classes.push(class);
    }

    let final_class = classes.join(" ");

    rsx! {
        div {
            class: final_class,
            for (label, value, stat_type) in props.stats.iter() {
                div {
                    class: "stat-item",
                    span {
                        class: if let Some(t) = stat_type { format!("stat-value {}", t) } else { "stat-value".to_string() },
                        "{value}"
                    }
                    span { class: "stat-label", "{label}" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TerminalGridProps {

    #[props(default = "1fr".to_string())]
    columns: String,


    #[props(default = "0.75rem".to_string())]
    gap: String,


    #[props(default)]
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TerminalGrid(props: TerminalGridProps) -> Element {
    let mut classes = vec!["terminal-grid"];

    if let Some(class) = &props.class {
        classes.push(class);
    }

    let final_class = classes.join(" ");
    let grid_style = format!(
        "grid-template-columns: {}; gap: {};",
        props.columns, props.gap
    );

    rsx! {
        div {
            class: final_class,
            style: grid_style,
            ..props.attributes,
            {props.children}
        }
    }
}
