use crate::utils::lib::use_controlled;
use dioxus::html::geometry::euclid::Rect;
use dioxus::html::geometry::Pixels;
use dioxus::prelude::*;
use std::ops::RangeInclusive;
use tracing;

#[derive(Debug, Clone, PartialEq)]
pub enum SliderValue {
    Single(f64),
    Range(f64, f64),
}

impl std::fmt::Display for SliderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SliderValue::Single(v) => write!(f, "{}", v),
            SliderValue::Range(start, end) => write!(f, "{}, {}", start, end),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {

    value: Option<Signal<SliderValue>>,


    #[props(default = SliderValue::Single(0.0))]
    default_value: SliderValue,


    #[props(default = 0.0)]
    min: f64,


    #[props(default = 100.0)]
    max: f64,


    #[props(default = 1.0)]
    step: f64,


    #[props(default)]
    disabled: ReadOnlySignal<bool>,


    #[props(default = true)]
    horizontal: bool,


    #[props(default)]
    inverted: bool,


    #[props(default)]
    on_value_change: Callback<SliderValue>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    let (value, set_value) = use_controlled(
        props.value,
        props.default_value.clone(),
        props.on_value_change,
    );

    let orientation = if props.horizontal {
        "horizontal"
    } else {
        "vertical"
    };
    let range = props.min..=props.max;

    let mut dragging = use_signal(|| false);

    let ctx = use_context_provider(|| SliderContext {
        value,
        set_value,
        min: props.min,
        max: props.max,
        step: props.step,
        disabled: props.disabled,
        horizontal: props.horizontal,
        inverted: props.inverted,
        range,
        dragging: dragging.into(),
    });

    let rect = use_signal(|| None);

    rsx! {
        div {
            role: "group",
            "data-disabled": props.disabled,
            "data-orientation": orientation,

            onmounted: move |evt| {
                let mut rect = rect.clone();
                spawn(async move {
                    if let Ok(r) = evt.data().get_client_rect().await {
                        rect.set(Some(r));
                    }
                });
            },
            onmousemove: move |e| {
                if !dragging() || (ctx.disabled)() {
                    return;
                }
                let Some(rect) = rect() else {
                    tracing::warn!("Slider rect is not (yet) set");
                    return;
                };
                let current_pos = if ctx.horizontal {
                    e.data().client_coordinates().x
                } else {
                    e.data().client_coordinates().y
                } as f64;
                let new_value = get_value_from_pointer(
                    current_pos,
                    &rect,
                    ctx.min,
                    ctx.max,
                    ctx.inverted,
                );
                let stepped = (new_value / ctx.step).round() * ctx.step;
                ctx.set_value.call(SliderValue::Single(stepped));
            },

            onmousedown: move |e| {
                if (ctx.disabled)() {
                    return;
                }
                let Some(rect) = rect() else {
                    tracing::warn!("Slider rect is not (yet) set");
                    return;
                };
                dragging.set(true);
                let current_pos = if ctx.horizontal {
                    e.data().client_coordinates().x
                } else {
                    e.data().client_coordinates().y
                } as f64;
                let new_value = get_value_from_pointer(
                    current_pos,
                    &rect,
                    ctx.min,
                    ctx.max,
                    ctx.inverted,
                );
                let stepped = (new_value / ctx.step).round() * ctx.step;
                ctx.set_value.call(SliderValue::Single(stepped));
            },

            onmouseup: move |_| dragging.set(false),
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SliderTrackProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn SliderTrack(props: SliderTrackProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    rsx! {
        div {
            "data-disabled": ctx.disabled,
            "data-orientation": orientation,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SliderRangeProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SliderRange(props: SliderRangeProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    let style = use_memo(move || {
        let (start, end) = match (ctx.value)() {
            SliderValue::Single(v) => (ctx.min, v),
            SliderValue::Range(start, end) => (start, end),
        };

        let start_percent = ((start - ctx.min) / (ctx.max - ctx.min) * 100.0).clamp(0.0, 100.0);
        let end_percent = ((end - ctx.min) / (ctx.max - ctx.min) * 100.0).clamp(0.0, 100.0);

        if ctx.horizontal {
            format!("left: {}%; right: {}%", start_percent, 100.0 - end_percent)
        } else {
            format!("bottom: {}%; top: {}%", start_percent, 100.0 - end_percent)
        }
    });

    rsx! {
        div {
            "data-disabled": ctx.disabled,
            "data-orientation": orientation,
            style,
            ..props.attributes,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SliderThumbProps {

    #[props(default)]
    index: Option<usize>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SliderThumb(props: SliderThumbProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    let value = use_memo(move || match ((ctx.value)(), props.index) {
        (SliderValue::Single(v), _) => v,
        (SliderValue::Range(start, _), Some(0)) => start,
        (SliderValue::Range(_, end), Some(1)) => end,
        _ => ctx.min,
    });

    let percent = ((value() - ctx.min) / (ctx.max - ctx.min) * 100.0).clamp(0.0, 100.0);
    let style = if ctx.horizontal {
        format!("left: {}%", percent)
    } else {
        format!("bottom: {}%", percent)
    };

    rsx! {
        button {
            r#type: "button",
            role: "slider",
            aria_valuemin: ctx.min,
            aria_valuemax: ctx.max,
            aria_valuenow: value,
            aria_orientation: orientation,
            "data-disabled": ctx.disabled,
            "data-orientation": orientation,
            "data-dragging": ctx.dragging,
            style,
            tabindex: 0,
            ..props.attributes,
        }
    }
}











fn linear_scale(input: [f64; 2], output: [f64; 2]) -> impl Fn(f64) -> f64 {
    let [in_min, in_max] = input;
    let [out_min, out_max] = output;

    move |x: f64| {

        let normalized = (x - in_min) / (in_max - in_min);


        out_min + normalized * (out_max - out_min)
    }
}














fn get_value_from_pointer(
    pointer_position: f64,
    rect: &Rect<f64, Pixels>,
    min: f64,
    max: f64,
    inverted: bool,
) -> f64 {
    let input = [0.0, rect.width()];
    let output = if !inverted { [min, max] } else { [max, min] };
    let value = linear_scale(input, output);

    value(pointer_position - rect.origin.x)
}

#[allow(dead_code)]
#[derive(Clone)]
struct SliderContext {
    value: Memo<SliderValue>,
    set_value: Callback<SliderValue>,
    min: f64,
    max: f64,
    step: f64,
    disabled: ReadOnlySignal<bool>,
    horizontal: bool,
    inverted: bool,
    range: RangeInclusive<f64>,
    dragging: ReadOnlySignal<bool>,
}
