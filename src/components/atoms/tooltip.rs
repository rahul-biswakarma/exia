use crate::utils::lib::{use_controlled, use_unique_id};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct TooltipCtx {

    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,


    tooltip_id: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {

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
pub fn Tooltip(props: TooltipProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let tooltip_id = use_unique_id();

    let _ctx = use_context_provider(|| TooltipCtx {
        open,
        set_open,
        disabled: props.disabled,
        tooltip_id,
    });

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TooltipTriggerProps {

    #[props(default)]
    id: Option<String>,


    #[props(default = true)]
    use_aria: bool,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let ctx: TooltipCtx = use_context();


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


    let handle_focus = move |_: Event<FocusData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_blur = move |_: Event<FocusData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };


    let handle_keydown = move |event: Event<KeyboardData>| {
        if event.key() == Key::Escape && (ctx.open)() {
            event.prevent_default();
            ctx.set_open.call(false);
        }
    };

    rsx! {
        div {
            id: props.id.clone(),

            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,

            onfocus: handle_focus,
            onblur: handle_blur,

            onkeydown: handle_keydown,

            aria_describedby: if props.use_aria { ctx.tooltip_id.peek().clone() } else { String::new() },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TooltipContentProps {

    #[props(default)]
    id: Option<String>,


    #[props(default = TooltipSide::Top)]
    side: TooltipSide,


    #[props(default = TooltipAlign::Center)]
    align: TooltipAlign,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TooltipSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
    fn as_str(self) -> &'static str {
        match self {
            TooltipSide::Top => "top",
            TooltipSide::Right => "right",
            TooltipSide::Bottom => "bottom",
            TooltipSide::Left => "left",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TooltipAlign {
    Start,
    Center,
    End,
}

impl TooltipAlign {
    fn as_str(self) -> &'static str {
        match self {
            TooltipAlign::Start => "start",
            TooltipAlign::Center => "center",
            TooltipAlign::End => "end",
        }
    }
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let ctx: TooltipCtx = use_context();


    let is_open = (ctx.open)();
    if !is_open {
        return rsx!({});
    }


    rsx! {
        div {
            id: props.id.clone().unwrap_or_else(|| ctx.tooltip_id.peek().clone()),
            role: "tooltip",
            "data-state": if is_open { "open" } else { "closed" },
            "data-side": props.side.as_str(),
            "data-align": props.align.as_str(),
            ..props.attributes,
            {props.children}
        }
    }
}
