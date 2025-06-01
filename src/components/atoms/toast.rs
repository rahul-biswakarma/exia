use crate::use_unique_id;
use dioxus::prelude::*;
use dioxus_time::use_timeout;
pub mod portal;
use portal::{use_portal, PortalIn, PortalOut};
use std::collections::VecDeque;
use std::time::Duration;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastType {
    fn as_str(&self) -> &'static str {
        match self {
            ToastType::Success => "success",
            ToastType::Error => "error",
            ToastType::Warning => "warning",
            ToastType::Info => "info",
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ToastItem {
    id: usize,
    title: String,
    description: Option<String>,
    toast_type: ToastType,
    duration: Option<Duration>,
    permanent: bool,
}


type AddToastCallback = Callback<(String, Option<String>, ToastType, Option<Duration>, bool)>;


#[derive(Clone)]
struct ToastCtx {
    #[allow(dead_code)]
    toasts: Signal<VecDeque<ToastItem>>,
    add_toast: AddToastCallback,
    remove_toast: Callback<usize>,
}


#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    #[props(default = ReadOnlySignal::new(Signal::new(Some(Duration::from_secs(5)))))]
    default_duration: ReadOnlySignal<Option<Duration>>,

    #[props(default = ReadOnlySignal::new(Signal::new(10)))]
    max_toasts: ReadOnlySignal<usize>,

    children: Element,
}


#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    let mut toasts = use_signal(VecDeque::new);
    let portal = use_portal();


    let ctx = ToastCtx {
        toasts,
        add_toast: Callback::new(|_| {}),
        remove_toast: Callback::new(|_| {}),
    };


    let remove_toast = Callback::new(move |id: usize| {
        let mut toasts_vec = toasts.write();
        if let Some(pos) = toasts_vec.iter().position(|t| t.id == id) {
            toasts_vec.remove(pos);
        }
    });


    let add_toast = Callback::new(
        move |(title, description, toast_type, duration, permanent): (
            String,
            Option<String>,
            ToastType,
            Option<Duration>,
            bool,
        )| {


            use std::sync::atomic::{AtomicUsize, Ordering};
            static NEXT_ID: AtomicUsize = AtomicUsize::new(0);


            let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);


            let duration = if permanent {
                None
            } else {
                duration.or_else(|| (props.default_duration)())
            };

            let toast = ToastItem {
                id,
                title,
                description,
                toast_type,
                duration,
                permanent,
            };



            let mut toasts_vec = toasts.write();
            toasts_vec.push_back(toast.clone());


            let max = (props.max_toasts)();
            while toasts_vec.len() > max {

                if let Some(pos) = toasts_vec.iter().position(|t| !t.permanent) {
                    toasts_vec.remove(pos);
                } else {

                    toasts_vec.pop_front();
                }
            }


        },
    );


    let mut ctx = ctx;
    ctx.add_toast = add_toast;
    ctx.remove_toast = remove_toast;


    let ctx = use_context_provider(|| ctx);


    let toast_list = use_memo(move || {
        let toasts_vec = toasts.read();
        toasts_vec.iter().cloned().collect::<Vec<_>>()
    });

    rsx! {

        {props.children}


        PortalIn { portal,
            div {
                role: "region",
                aria_live: "polite",
                aria_label: "Notifications",
                class: "toast-container",


                for toast in toast_list().iter() {
                    Toast {
                        key: format!("{}", toast.id),
                        id: toast.id,
                        title: toast.title.clone(),
                        description: toast.description.clone(),
                        toast_type: toast.toast_type,
                        permanent: toast.permanent,
                        on_close: {
                            let toast_id = toast.id;
                            let remove_toast = ctx.remove_toast;
                            move |_| {
                                remove_toast.call(toast_id);
                            }
                        },


                        duration: if toast.permanent { None } else { toast.duration },
                    }
                }
            }
        }


        PortalOut { portal }
    }
}


#[derive(Props, Clone, PartialEq)]
pub struct ToastProps {
    id: usize,
    title: String,
    description: Option<String>,
    toast_type: ToastType,
    on_close: Callback<MouseEvent>,
    #[props(default = false)]
    permanent: bool,

    duration: Option<Duration>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}


#[component]
pub fn Toast(props: ToastProps) -> Element {
    let toast_id = use_unique_id();
    let id = use_memo(move || format!("toast-{}", toast_id()));


    let ctx = use_context::<ToastCtx>();



    if !props.permanent && props.duration.is_some() {
        let duration = props.duration.unwrap();
        let toast_id = props.id;
        let remove_toast = ctx.remove_toast;


        let timeout = use_timeout(duration, move |()| {

            remove_toast.call(toast_id);
        });


        use_effect(move || {
            timeout.action(());
        });
    }

    rsx! {
        div {
            id,
            role: "alert",
            class: "toast",
            "data-type": props.toast_type.as_str(),
            "data-permanent": props.permanent.to_string(),
            ..props.attributes,

            div { class: "toast-content",

                div { class: "toast-title", {props.title.clone()} }

                if let Some(description) = &props.description {
                    div { class: "toast-description", {description.clone()} }
                }
            }

            button {
                class: "toast-close",
                aria_label: "Close",
                onclick: move |e| props.on_close.call(e),
                "×"
            }
        }
    }
}


#[derive(Clone, Default)]
pub struct ToastOptions {
    pub description: Option<String>,
    pub duration: Option<Duration>,
    pub permanent: bool,
}


type AddToastFn = AddToastCallback;


#[derive(Clone, Copy)]
pub struct Toasts {
    add_toast: AddToastFn,

    #[allow(dead_code)]
    remove_toast: Callback<usize>,
}

impl Toasts {

    pub fn show(&self, title: String, toast_type: ToastType, options: ToastOptions) {
        self.add_toast.call((
            title,
            options.description,
            toast_type,

            if options.permanent {
                None
            } else {
                options.duration
            },
            options.permanent,
        ));
    }


    pub fn success(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Success, options.unwrap_or_default());
    }

    pub fn error(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Error, options.unwrap_or_default());
    }

    pub fn warning(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Warning, options.unwrap_or_default());
    }

    pub fn info(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Info, options.unwrap_or_default());
    }
}


pub fn use_toast() -> Toasts {
    let ctx = use_context::<ToastCtx>();
    let add_toast = ctx.add_toast;
    let remove_toast = ctx.remove_toast;

    Toasts {
        add_toast,
        remove_toast,
    }
}
