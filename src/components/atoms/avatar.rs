use dioxus::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarState {

    Loading,

    Loaded,

    Error,

    Empty,
}

#[derive(Clone)]
struct AvatarCtx {

    state: Signal<AvatarState>,
    has_fallback_child: Signal<bool>,
    has_image_child: Signal<bool>,


    on_load: Option<EventHandler<()>>,
    on_error: Option<EventHandler<()>>,
    on_state_change: Option<EventHandler<AvatarState>>,
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {

    #[props(default)]
    pub on_load: Option<EventHandler<()>>,


    #[props(default)]
    pub on_error: Option<EventHandler<()>>,


    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {

    let state = use_signal(|| AvatarState::Empty);
    let has_fallback_child = use_signal(|| false);
    let has_image_child = use_signal(|| false);


    use_effect(move || {
        if let Some(handler) = &props.on_state_change {
            handler.call(state());
        }
    });


    let _ctx = use_context_provider(|| AvatarCtx {
        state,
        has_fallback_child,
        has_image_child,
        on_load: props.on_load,
        on_error: props.on_error,
        on_state_change: props.on_state_change,
    });


    let show_fallback =
        use_memo(move || matches!(state(), AvatarState::Error | AvatarState::Empty));

    rsx! {
        span {
            role: "img",
            "data-state": match state() {
                AvatarState::Loading => "loading",
                AvatarState::Loaded => "loaded",
                AvatarState::Error => "error",
                AvatarState::Empty => "empty",
            },
            ..props.attributes,


            {props.children}


            if show_fallback() && !has_fallback_child() && has_image_child() {
                span { style: "display: flex; align-items: center; justify-content: center; width: 100%; height: 100%;",
                    "??"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let mut ctx: AvatarCtx = use_context();


    use_effect(move || {
        ctx.has_fallback_child.set(true);
    });

    let show_fallback =
        use_memo(move || matches!((ctx.state)(), AvatarState::Error | AvatarState::Empty));

    if !show_fallback() {
        return rsx!({});
    }

    rsx! {
        span { ..props.attributes,{props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {

    pub src: String,


    #[props(default)]
    pub alt: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let mut ctx: AvatarCtx = use_context();


    use_effect(move || {
        ctx.has_image_child.set(true);
        ctx.state.set(AvatarState::Loading);
    });

    let handle_load = move |_| {
        ctx.state.set(AvatarState::Loaded);
        if let Some(handler) = &ctx.on_load {
            handler.call(());
        }
        if let Some(handler) = &ctx.on_state_change {
            handler.call(AvatarState::Loaded);
        }
    };

    let handle_error = move |_| {
        ctx.state.set(AvatarState::Error);
        if let Some(handler) = &ctx.on_error {
            handler.call(());
        }
        if let Some(handler) = &ctx.on_state_change {
            handler.call(AvatarState::Error);
        }
    };

    rsx! {
        img {
            src: props.src.clone(),
            alt: props.alt.clone().unwrap_or_default(),
            onload: handle_load,
            onerror: handle_error,
            style: "width: 100%; height: 100%; object-fit: cover;",
            ..props.attributes,
        }
    }
}
