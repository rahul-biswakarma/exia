use crate::utils::lib::{use_id_or, use_unique_id};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {

    #[props(default)]
    value: Option<Signal<Option<String>>>,


    #[props(default)]
    default_value: Option<String>,


    #[props(default)]
    on_value_change: Callback<Option<String>>,


    #[props(default)]
    disabled: ReadOnlySignal<bool>,


    #[props(default)]
    required: ReadOnlySignal<bool>,


    #[props(default)]
    name: ReadOnlySignal<String>,


    #[props(default)]
    id: ReadOnlySignal<Option<String>>,


    #[props(default = String::from("Select an option"))]
    placeholder: String,


    #[props(default)]
    aria_label: Option<String>,


    #[props(default)]
    aria_labelledby: Option<String>,


    #[props(default)]
    aria_describedby: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {

    let mut internal_value = use_signal(|| props.value.map(|x| x()).unwrap_or(props.default_value));


    let select_id = use_unique_id();


    let id_value = use_id_or(select_id, props.id);


    let handle_change = move |event: Event<FormData>| {
        let value = event.value();
        let new_value = if value.is_empty() { None } else { Some(value) };
        internal_value.set(new_value.clone());
        props.on_value_change.call(new_value);
    };


    let current_value = props.value.map(|v| v()).unwrap_or_else(|| internal_value());


    let has_selection = current_value.is_some();


    let active_option_id =
        has_selection.then(|| format!("option-{}", current_value.clone().unwrap()));

    rsx! {
        select {

            id: id_value,
            class: "select",
            name: props.name,
            disabled: (props.disabled)(),
            required: (props.required)(),


            value: current_value.clone().unwrap_or_default(),
            onchange: handle_change,


            role: "combobox",
            aria_haspopup: "listbox",
            aria_expanded: "false",
            aria_autocomplete: "none",
            aria_required: (props.required)().to_string(),
            aria_label: props.aria_label.clone(),
            aria_labelledby: props.aria_labelledby.clone(),
            aria_describedby: props.aria_describedby.clone(),
            aria_invalid: "false",
            aria_activedescendant: active_option_id,


            ..props.attributes,


            if current_value.is_none() {
                option {
                    value: "",
                    selected: true,
                    disabled: true,
                    role: "option",
                    aria_selected: "false",
                    {props.placeholder}
                }
            }


            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectOptionProps {

    value: String,


    #[props(default)]
    disabled: ReadOnlySignal<bool>,


    #[props(default)]
    id: ReadOnlySignal<Option<String>>,


    #[props(default)]
    aria_label: Option<String>,


    #[props(default)]
    aria_roledescription: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectOption(props: SelectOptionProps) -> Element {

    let option_id = use_signal(|| format!("option-{}", props.value));


    let id = use_id_or(option_id, props.id);

    rsx! {
        option {
            id,
            value: props.value.clone(),
            disabled: (props.disabled)(),


            role: "option",
            aria_selected: "false",
            aria_disabled: (props.disabled)().to_string(),
            aria_label: props.aria_label.clone(),
            aria_roledescription: props.aria_roledescription.clone(),

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupProps {

    label: String,


    #[props(default)]
    disabled: ReadOnlySignal<bool>,


    #[props(default)]
    id: ReadOnlySignal<Option<String>>,


    #[props(default)]
    aria_label: Option<String>,


    #[props(default)]
    aria_roledescription: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {

    let group_id = use_signal(|| format!("group-{}", props.label.to_lowercase().replace(" ", "-")));


    let id = use_id_or(group_id, props.id);

    rsx! {
        optgroup {
            id,
            label: props.label.clone(),
            disabled: (props.disabled)(),


            role: "group",
            aria_disabled: (props.disabled)().to_string(),
            aria_label: props.aria_label.clone(),
            aria_roledescription: props.aria_roledescription.clone(),

            ..props.attributes,
            {props.children}
        }
    }
}
