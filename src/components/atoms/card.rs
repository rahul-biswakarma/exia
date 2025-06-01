use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {

    #[props(default = false)]
    decorated: bool,


    #[props(default = false)]
    glow: bool,


    #[props(default = true)]
    hoverable: bool,


    #[props(default)]
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let mut classes = vec!["card"];


    if props.decorated {
        classes.push("card-decorated");
    }

    if props.glow {
        classes.push("card-glow");
    }


    if props.hoverable {
        classes.push("card-hoverable");
    }

    if let Some(class) = &props.class {
        classes.push(class);
    }

    let final_class = classes.join(" ");

    rsx! {
        div {
            class: final_class,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        div {
            class: "card-header",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    children: Element,
}

#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    rsx! {
        div {
            class: "card-content",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    rsx! {
        div {
            class: "card-footer",
            {props.children}
        }
    }
}
