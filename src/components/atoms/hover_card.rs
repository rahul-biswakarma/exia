use crate::utils::lib::{use_controlled, use_id_or, use_unique_id};
use dioxus::prelude::*;

#[derive(Clone)]
struct HoverCardCtx {

    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,


    content_id: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct HoverCardProps {

    open: Option<Signal<bool>>,


    #[props(default)]
    default_open: bool,


    #[props(default)]
    on_open_change: Callback<bool>,


    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let content_id = use_unique_id();

    let _ctx = use_context_provider(|| HoverCardCtx {
        open,
        set_open,
        disabled: props.disabled,
        content_id,
    });

    rsx! {
        div {
            class: "hover-card",
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct HoverCardTriggerProps {

    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    let ctx: HoverCardCtx = use_context();


    let trigger_id = use_unique_id();


    let id = use_id_or(trigger_id, props.id);


    let handle_mouse_enter = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_mouse_leave = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    rsx! {
        div {
            id,
            class: "hover-card-trigger",


            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,


            aria_haspopup: "dialog",
            aria_expanded: (ctx.open)(),
            aria_controls: ctx.content_id.peek().clone(),

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HoverCardSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl HoverCardSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            HoverCardSide::Top => "top",
            HoverCardSide::Right => "right",
            HoverCardSide::Bottom => "bottom",
            HoverCardSide::Left => "left",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HoverCardAlign {
    Start,
    Center,
    End,
}

impl HoverCardAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            HoverCardAlign::Start => "start",
            HoverCardAlign::Center => "center",
            HoverCardAlign::End => "end",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct HoverCardContentProps {

    #[props(default)]
    id: ReadOnlySignal<Option<String>>,


    #[props(default = HoverCardSide::Top)]
    side: HoverCardSide,


    #[props(default = HoverCardAlign::Center)]
    align: HoverCardAlign,


    #[props(default = true)]
    force_mount: bool,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    let ctx: HoverCardCtx = use_context();


    let is_open = (ctx.open)();
    if !is_open && !props.force_mount {
        return rsx!({});
    }


    let id = use_id_or(ctx.content_id, props.id);


    let handle_mouse_enter = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_mouse_leave = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    rsx! {
        div {
            id,
            class: "hover-card-content",
            role: "dialog",
            "data-state": if is_open { "open" } else { "closed" },
            "data-side": props.side.as_str(),
            "data-align": props.align.as_str(),


            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,

            ..props.attributes,
            {props.children}
        }
    }
}
