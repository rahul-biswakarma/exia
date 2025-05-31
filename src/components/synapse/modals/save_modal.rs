use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SaveModalProps {
    pub show_save_modal: Signal<bool>,
    pub generated_ui: Signal<Option<serde_json::Value>>,
    pub prompt: Signal<String>,
}

#[component]
pub fn SaveModal(mut props: SaveModalProps) -> Element {
    if !(props.show_save_modal)() {
        return rsx! { div {} };
    }

    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-white rounded-lg p-6 max-w-md w-full mx-4",
                h3 { class: "text-lg font-semibold mb-4", "Save Schema" }

                div { class: "text-center text-gray-500 py-4",
                    p { "Schema saving functionality will be available soon." }
                    p { class: "text-sm mt-2", "This feature requires database integration." }
                }

                div { class: "flex gap-2 mt-6",
                    button {
                        class: "flex-1 px-4 py-2 bg-gray-200 text-gray-800 rounded-lg hover:bg-gray-300 transition-colors",
                        onclick: move |_| props.show_save_modal.set(false),
                        "Close"
                    }
                }
            }
        }
    }
}
