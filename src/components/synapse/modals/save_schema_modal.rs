use dioxus::prelude::*;

#[component]
pub fn SaveSchemaModal() -> Element {
    rsx! {
        div { class: "text-center text-gray-500 py-4",
            p { "Schema saving functionality will be available soon." }
            p { class: "text-sm mt-2", "This feature requires database integration." }
        }
    }
}
