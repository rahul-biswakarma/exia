use crate::utils::lib::use_controlled;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct ContextMenuCtx {

    open: ReadOnlySignal<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,


    position: Signal<(i32, i32)>,


    item_count: Signal<usize>,
    recent_focus: Signal<usize>,
    current_focus: Signal<Option<usize>>,
}

impl ContextMenuCtx {
    fn set_focus(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            self.recent_focus.set(idx);
        }
        self.current_focus.set(index);
    }

    fn focus_next(&mut self) {
        let count = *self.item_count.read();
        if count == 0 {
            return;
        }

        let next = match *self.current_focus.read() {
            Some(current) => (current + 1) % count,
            None => 0,
        };
        self.set_focus(Some(next));
    }

    fn focus_prev(&mut self) {
        let count = *self.item_count.read();
        if count == 0 {
            return;
        }

        let prev = match *self.current_focus.read() {
            Some(current) => {
                if current == 0 {
                    count - 1
                } else {
                    current - 1
                }
            }
            None => count - 1,
        };
        self.set_focus(Some(prev));
    }

    fn focus_first(&mut self) {
        if *self.item_count.read() > 0 {
            self.set_focus(Some(0));
        }
    }

    fn focus_last(&mut self) {
        let count = *self.item_count.read();
        if count > 0 {
            self.set_focus(Some(count - 1));
        }
    }


    fn restore_trigger_focus(&mut self) {


        self.current_focus.set(None);
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps {

    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    disabled: ReadOnlySignal<bool>,


    open: Option<Signal<bool>>,


    #[props(default)]
    default_open: bool,


    #[props(default)]
    on_open_change: Callback<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let position = use_signal(|| (0, 0));

    let mut ctx = use_context_provider(|| ContextMenuCtx {
        open: open.into(),
        set_open,
        disabled: props.disabled,
        position,
        item_count: Signal::new(0),
        recent_focus: Signal::new(0),
        current_focus: Signal::new(None),
    });

    let handle_click = move |event: Event<MouseData>| {
        if open() {
            let coords = event.data().client_coordinates();
            let click_x = coords.x as i32;
            let click_y = coords.y as i32;
            let (menu_x, menu_y) = position();


            let menu_width = 200;
            let menu_height = 200;

            if click_x < menu_x
                || click_x > menu_x + menu_width
                || click_y < menu_y
                || click_y > menu_y + menu_height
            {
                set_open.call(false);
                ctx.restore_trigger_focus();
            }
        }
    };


    let handle_keydown = move |event: Event<KeyboardData>| {
        if open() && event.key() == Key::Escape {
            event.prevent_default();
            set_open.call(false);
            ctx.restore_trigger_focus();
        }
    };

    rsx! {
        div {
            onclick: handle_click,
            onkeydown: handle_keydown,
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuTriggerProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();

    let handle_context_menu = move |event: Event<MouseData>| {
        if !(ctx.disabled)() {
            event.prevent_default();
            ctx.position.set((
                event.data().client_coordinates().x as i32,
                event.data().client_coordinates().y as i32,
            ));
            ctx.set_open.call(true);
        }
    };

    rsx! {
        div {
            oncontextmenu: handle_context_menu,
            aria_haspopup: "menu",
            aria_expanded: (ctx.open)(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();
    let position = ctx.position;

    let style = use_memo(move || {
        let (x, y) = position();
        format!("position: fixed; left: {}px; top: {}px;", x, y)
    });


    let is_open = (ctx.open)();
    use_effect(move || {
        if is_open {
            ctx.focus_first();
        }
    });

    rsx! {
        div {
            role: "menu",
            aria_orientation: "vertical",
            style: "{style}",
            "data-state": if (ctx.open)() { "open" } else { "closed" },
            hidden: !(ctx.open)(),
            onclick: move |e| e.stop_propagation(),
            onkeydown: move |event: Event<KeyboardData>| {
                let mut prevent_default = true;
                match event.key() {
                    Key::ArrowDown => ctx.focus_next(),
                    Key::ArrowUp => ctx.focus_prev(),
                    Key::Home => ctx.focus_first(),
                    Key::End => ctx.focus_last(),
                    Key::Escape => {
                        ctx.set_open.call(false);
                        ctx.restore_trigger_focus();
                    }
                    _ => prevent_default = false,
                }
                if prevent_default {
                    event.prevent_default();
                }
            },
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuItemProps {

    value: ReadOnlySignal<String>,


    index: ReadOnlySignal<usize>,


    #[props(default)]
    on_select: Callback<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();


    use_effect(move || {
        ctx.item_count += 1;
    });


    use_drop(move || {
        ctx.item_count -= 1;
        if (ctx.current_focus)() == Some((props.index)()) {
            ctx.set_focus(None);
        }
    });


    let tab_index = use_memo(move || {
        if (ctx.current_focus)() == Some((props.index)()) {
            "0"
        } else {
            "-1"
        }
    });

    let handle_click = {
        let value = (props.value)().clone();
        move |_| {
            if !(ctx.disabled)() {
                props.on_select.call(value.clone());
                ctx.set_open.call(false);
                ctx.restore_trigger_focus();
            }
        }
    };

    let handle_keydown = {
        let value = (props.value)().clone();
        move |event: Event<KeyboardData>| {

            if event.key() == Key::Enter || event.key().to_string() == " " {
                event.prevent_default();
                if !(ctx.disabled)() {
                    props.on_select.call(value.clone());
                    ctx.set_open.call(false);
                    ctx.restore_trigger_focus();
                }
            }
        }
    };

    rsx! {
        div {
            role: "menuitem",
            tabindex: tab_index,
            onclick: handle_click,
            onkeydown: handle_keydown,
            onfocus: move |_| ctx.set_focus(Some((props.index)())),
            aria_disabled: (ctx.disabled)(),
            ..props.attributes,

            {props.children}
        }
    }
}
