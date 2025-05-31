use dioxus::prelude::*;

#[component]
pub fn SavedSchemas() -> Element {
    rsx! {
        div { class: "bg-white rounded-xl shadow-lg p-6",
            h3 { class: "text-lg font-semibold text-gray-800 mb-4",
                "📚 Your Saved Schemas"
            }
            div { class: "text-center text-gray-500 py-8",
                p { "Schema saving functionality will be available soon." }
                p { class: "text-sm mt-2", "Sign in to save and manage your UI schemas." }
            }
        }
    }
}
