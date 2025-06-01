use crate::auth::AuthContext;
use crate::components::auth::AuthGuard;
use crate::components::themes::context::ThemeProvider;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FONTS_CSS: Asset = asset!("/assets/styles/fonts.css");
const DESIGN_SYSTEM_CSS: Asset = asset!("/assets/styles/design-system.css");
const NEON_EVANGELION_CSS: Asset = asset!("/assets/styles/themes/neon-evangelion.css");

#[component]
pub fn App() -> Element {
    let auth_context = use_signal(|| AuthContext::new());
    use_context_provider(|| auth_context);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: FONTS_CSS }
        document::Link { rel: "stylesheet", href: DESIGN_SYSTEM_CSS }
        document::Link { rel: "stylesheet", href: NEON_EVANGELION_CSS }
        ThemeProvider { AuthGuard {} }
    }
}
