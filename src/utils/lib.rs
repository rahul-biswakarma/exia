use dioxus::prelude::*;

pub fn use_unique_id() -> Signal<String> {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    let id = *NEXT_ID.peek();
    let id_str = format!("dxc-{id}");

    use_effect(move || {
        *NEXT_ID.write() += 1;
    });

    use_signal(|| id_str)
}

pub fn use_id_or(
    mut gen_id: Signal<String>,
    user_id: ReadOnlySignal<Option<String>>,
) -> Memo<String> {
    let has_user_id = use_memo(move || user_id().is_some());

    use_effect(move || {
        if let Some(id) = user_id() {
            gen_id.set(id);
        }
    });

    use_memo(move || {
        if has_user_id() {
            user_id().unwrap()
        } else {
            gen_id.peek().clone()
        }
    })
}

pub fn use_controlled<T: Clone + PartialEq>(
    prop: Option<Signal<T>>,
    default: T,
    on_change: Callback<T>,
) -> (Memo<T>, Callback<T>) {
    let mut internal_value = use_signal(|| prop.map(|x| x()).unwrap_or(default));
    let value = use_memo(move || prop.unwrap_or(internal_value)());

    let set_value = Callback::new(move |x: T| {
        internal_value.set(x.clone());
        on_change.call(x);
    });

    (value, set_value)
}
